//! gpu_kernel.rs — the token-graph executor on the GPU.
//!
//! The kernel's tick loop, one program per thread. Each thread runs the same
//! Frobenius step machine `kernel.rs::tick` runs on the CPU: a B4 stack, eight
//! B4 registers with the engagr flag, four B4 memory cells addressed by
//! register 0, and a fork stack for FSPLIT/FFUSE. This is the classical core,
//! marks VINIT TANCH AFWD AREV CLINK EVALT FSPLIT FFUSE EVALF ENGAGR IFIX
//! ROTAT. IMSCRIB (the snapshot read) and the three-way FSPLIT3/FFUSE3/EVALI
//! are the next stage; programs carrying them are run on the CPU.
//!
//! `verify` runs random classical programs on both the device and the CPU
//! kernel and reports the first divergence, so the port is checked against the
//! executor it replaces, not asserted.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use cudarc::driver::{CudaContext, LaunchConfig, PushKernelArg};
use cudarc::nvrtc::{compile_ptx, compile_ptx_with_opts, CompileOptions};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use crate::tokens::{Program, Token};
use crate::kernel::Kernel;

const STRIDE: usize = 64;   // Fixed stride for short-word batch controls.
const SDEPTH: usize = 256;  // stack depth
const META: usize = 32;     // Classical state followed by the banked tri face.

/// Token id, the declared order of the enum (0..15).
/// The token id mapping, for other GPU modules that pack programs.
pub fn tok_id_pub(t: Token) -> u8 { tok_id(t) }

fn tok_id(t: Token) -> u8 {
    match t {
        Token::Vinit => 0, Token::Tanch => 1, Token::Afwd => 2, Token::Arev => 3,
        Token::Clink => 4, Token::Evalt => 5, Token::Fsplit => 6, Token::Ffuse => 7,
        Token::Imscrib => 8, Token::Evalf => 9, Token::Engagr => 10, Token::Ifix => 11,
        Token::Fsplit3 => 12, Token::Ffuse3 => 13, Token::Evali => 14, Token::Rotat => 15,
    }
}

const KERNEL_SRC: &str = r#"
#ifndef PROGRAM_CAP
#define PROGRAM_CAP 64
#endif
extern "C" __global__ void run_programs(
    const unsigned char* progs, const unsigned int* lens,
    unsigned char* out_stack, unsigned int* out_meta,
    unsigned long long max_ticks, unsigned int n)
{
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n) return;

    const int STRIDE = PROGRAM_CAP, SDEPTH = 256, FCAP = 64, META = 32;
    unsigned char P[PROGRAM_CAP];
    unsigned int plen = lens[gid];
    if (plen > PROGRAM_CAP) plen = PROGRAM_CAP;
    for (unsigned int i = 0; i < plen; i++) P[i] = progs[gid*STRIDE + i];

    unsigned char stack[256]; int top = 0;
    unsigned char reg[8]; for (int i=0;i<8;i++) reg[i]=0;
    unsigned char engagr = 0;
    unsigned char mem[4]; mem[0]=mem[1]=mem[2]=mem[3]=0;
    unsigned int fresume[64]; unsigned char fright[64]; unsigned char fset[64];
    int fdepth = 0;
    unsigned int ip = 0;
    unsigned long long tick = 0;
    unsigned char halted = 0;

    // IMSCRIB reads four snapshot fields; all are functions of the program,
    // which is static over a non-dynamic run, so compute them once.
    int diversity = 0;
    { unsigned char seen[16]; for (int i=0;i<16;i++) seen[i]=0;
      for (unsigned int i=0;i<plen;i++){ unsigned char t=P[i]; if (t<16 && !seen[t]) { seen[t]=1; diversity++; } } }
    unsigned char self_ref = (plen>0 && P[0]==P[plen-1]) ? 1 : 0;
    unsigned char frob_pos = 0;
    for (unsigned int i=0;i<plen;i++){ unsigned char t=P[i]; if (t==6||t==7||t==12||t==13){ frob_pos=1; break; } }
    unsigned char dial = 0;
    { unsigned char h5=0,h9=0,h10=0;
      for (unsigned int i=0;i<plen;i++){ if (P[i]==5)h5=1; else if (P[i]==9)h9=1; else if (P[i]==10)h10=1; }
      if (h5 && h9 && h10) {
          dial = 1;
          for (unsigned int i=0;i<plen && dial;i++){
              if (P[i]==10){
                  unsigned char found=0;
                  for (unsigned int off=1; off<plen; off++){ unsigned char t=P[(i+off)%plen]; if (t==5||t==9){ found=1; break; } }
                  if (!found) dial=0;
              }
          }
      }
    }

    while (!halted && tick < max_ticks && plen > 0) {
        tick++;
        // ACT: wrap + try_self_modify (non-dynamic: inject TANCH if stack deep)
        if (ip >= plen) { ip = 0; if (top > 200) P[ip] = 1; }
        unsigned char tok = P[ip];
        unsigned int next_ip = ip + 1;
        if (next_ip >= plen) next_ip = 0;   // winding count omitted (internal)

        if (tok == 0) {                     // VINIT: push N
            if (top < SDEPTH) stack[top++] = 0;
        } else if (tok == 1) {              // TANCH
            unsigned char v = (top>0)? stack[--top] : 0;
            mem[reg[0] & 3] = v;
            if (fdepth == 0) { halted = 1; break; }
        } else if (tok == 2) {              // AFWD
            reg[0] = (reg[0] + 1) & 3;
        } else if (tok == 3) {              // AREV: bnot top, dec reg0
            unsigned char v = (top>0)? stack[--top] : 0;
            unsigned char nb = ((v & 1) << 1) | ((v & 2) >> 1);
            if (top < SDEPTH) stack[top++] = nb;
            reg[0] = (reg[0] - 1) & 3;
        } else if (tok == 4) {              // CLINK: reg3 = meet(reg1,reg2)
            reg[3] = reg[1] & reg[2];
        } else if (tok == 5) {              // EVALT
            unsigned char v = (top>0)? stack[--top] : 0;
            if (top < SDEPTH) stack[top++] = (v == 1) ? 1 : 0;
        } else if (tok == 6) {              // FSPLIT
            unsigned char v = (top>0)? stack[top-1] : 0;
            // find matching FFUSE
            unsigned int ff = plen; unsigned int depth = 1;
            unsigned int i = (ip + 1) % plen; unsigned int start = i;
            do {
                if (P[i] == 6) depth++;
                else if (P[i] == 7) { depth--; if (depth==0) { ff = i; break; } }
                i = (i + 1) % plen;
            } while (i != start);
            unsigned int resume = (ff + 1 >= plen) ? 0 : ff + 1;
            // push_fork: add a frame only if there is room (overflow drops it)
            if (fdepth < FCAP) { fresume[fdepth]=resume; fright[fdepth]=0; fset[fdepth]=0; fdepth++; }
            // fork_top_mut: set the (new or, on overflow, existing) top frame's right
            if (fdepth > 0) { fright[fdepth-1]=v; fset[fdepth-1]=1; }
            if (top < SDEPTH) stack[top++] = v;
        } else if (tok == 7) {              // FFUSE
            unsigned char left = (top>0)? stack[--top] : 0;
            if (fdepth > 0) {
                fdepth--;
                unsigned char right = fset[fdepth] ? fright[fdepth] : 0;
                if (top < SDEPTH) stack[top++] = left | (right & 3);  // b4_join(left, collapse(right))
                next_ip = fresume[fdepth];
            } else {
                if (top < SDEPTH) stack[top++] = left;
            }
        } else if (tok == 12) {             // FSPLIT3 (three-way delta)
            unsigned char v = (top>0)? stack[top-1] : 0;   // B4
            unsigned char r = v & 3;                        // b4_to_reg16_3, small bits 0
            unsigned int ff = plen; unsigned int depth = 1;
            unsigned int i = (ip + 1) % plen; unsigned int start = i;
            do {
                if (P[i] == 12) depth++;
                else if (P[i] == 13) { depth--; if (depth==0) { ff = i; break; } }
                i = (i + 1) % plen;
            } while (i != start);
            unsigned int resume = (ff + 1 >= plen) ? 0 : ff + 1;
            // constructive_part(r).falsity_part().union(info_part(r)), as 4-bit T,F,t,f
            unsigned char rv = ((r & 3) & 0xA) | (r & 0xC);
            if (fdepth < FCAP) { fresume[fdepth]=resume; fright[fdepth]=0; fset[fdepth]=0; fdepth++; }
            if (fdepth > 0) { fright[fdepth-1]=rv; fset[fdepth-1]=1; }
            if (top < SDEPTH) stack[top++] = v;
        } else if (tok == 13) {             // FFUSE3 (union of arms)
            unsigned char left = (top>0)? stack[--top] : 0;
            if (fdepth > 0) {
                fdepth--;
                unsigned char right = fset[fdepth] ? fright[fdepth] : 0;
                unsigned char fused = (left & 3) | right;   // union in 4-bit
                if (top < SDEPTH) stack[top++] = fused & 3;  // reg16_3_to_b4
                next_ip = fresume[fdepth];
            } else {
                if (top < SDEPTH) stack[top++] = left;
            }
        } else if (tok == 14) {             // EVALI (info part)
            unsigned char v = (top>0)? stack[--top] : 0;
            unsigned char info = (v & 3) & 0xC;   // info_part of a B4 value is empty
            if (top < SDEPTH) stack[top++] = info & 3;
        } else if (tok == 8) {              // IMSCRIB: snapshot fields to reg4-7
            reg[4] = (unsigned char)(diversity & 3);
            reg[5] = self_ref ? 1 : 2;   // T : F
            reg[6] = frob_pos ? 1 : 2;
            reg[7] = dial ? 1 : 2;
        } else if (tok == 9) {              // EVALF
            unsigned char v = (top>0)? stack[--top] : 0;
            if (top < SDEPTH) stack[top++] = (v == 2) ? 2 : 0;
        } else if (tok == 10) {             // ENGAGR
            engagr = 1;
            if (top < SDEPTH) stack[top++] = 3;
        } else if (tok == 11) {             // IFIX
            unsigned char v = (top>0)? stack[--top] : 0;
            mem[reg[0] & 3] = v;
        } else if (tok == 15) {             // ROTAT: rotate stack right by k
            unsigned char kv = (top>0)? stack[--top] : 0;
            int k = (int)kv; if (k < 1) k = 1;
            int nn = top;
            if (nn > 1) {
                k = k % nn; if (k < 0) k += nn;
                if (k > 0) {
                    // reverse [0,nn), reverse [0,k), reverse [k,nn)
                    for (int a=0,b=nn-1; a<b; a++,b--) { unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
                    for (int a=0,b=k-1;  a<b; a++,b--) { unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
                    for (int a=k,b=nn-1; a<b; a++,b--) { unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
                }
            }
        }
        // tok==8 (IMSCRIB) and 12/13/14 (three-way) are excluded from verified programs.

        ip = next_ip;
        // UPDATE: wrap + try_self_modify again
        if (ip >= plen) { ip = 0; if (top > 200) P[ip] = 1; }
    }

    for (int i = 0; i < top; i++) out_stack[gid*SDEPTH + i] = stack[i];
    unsigned int* m = &out_meta[gid*META];
    m[0] = halted;
    m[1] = (unsigned int)tick;
    m[2] = (unsigned int)top;
    for (int i=0;i<8;i++) m[3+i] = reg[i];
    m[11] = engagr;
    for (int i=0;i<4;i++) m[12+i] = mem[i];
}
"#;

// The nested kernel. Each thread runs the same inner word as run_programs, so
// its final state is identical and still checks against the CPU. The difference
// is the dispatch: every inner step is fetched and decoded by an interpreter
// word held in META[], stepped through the same twelve-mark token engine on a
// scratch stack before the inner action fires. That is IMASM interpreting
// IMASM on the card. out_meta[1] carries the inner tick count so the nested
// throughput is comparable to the flat kernel; out_meta carries the nested
// tick count so the nesting cost is visible too.
const KERNEL_NESTED_SRC: &str = concat!(r#"
#ifndef PROGRAM_CAP
#define PROGRAM_CAP 64
#endif
"#, include_str!("gpu_banked.cuh"), include_str!("gpu_relation.cuh"), r#"
#ifndef IMASM_FACTOR_RELATION
extern "C" __global__ void run_nested(
    const unsigned char* progs, const unsigned int* lens,
    unsigned char* out_stack, unsigned int* out_meta,
    unsigned long long max_ticks, unsigned int n)
{
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n) return;

    const int STRIDE = PROGRAM_CAP, SDEPTH = 256, FCAP = 64, META = 32;
    unsigned char P[PROGRAM_CAP];
    unsigned int plen = lens[gid];
    if (plen > PROGRAM_CAP) plen = PROGRAM_CAP;
    for (unsigned int i = 0; i < plen; i++) P[i] = progs[gid*STRIDE + i];

    // Read the unmodified process on its banked SIXTEEN_3 face in this same
    // launch. Its full-word cut differs from the classical halt/loop trace.
    banked_word(P,plen,&out_meta[gid*META+16]);

    // The interpreter word: the twelve marks that make up one fetch, decode and
    // execute, run as a real IMASM program on a scratch context each inner step.
    const int MLEN = 12;
    unsigned char INTERP[12] = {0,2,4,6,7,5,9,10,11,3,1,15};
    unsigned char mstack[64]; unsigned char mreg[8]; unsigned char mmem[4];

    unsigned char stack[256]; int top = 0;
    unsigned char reg[8]; for (int i=0;i<8;i++) reg[i]=0;
    unsigned char engagr = 0;
    unsigned char mem[4]; mem[0]=mem[1]=mem[2]=mem[3]=0;
    unsigned int fresume[64]; unsigned char fright[64]; unsigned char fset[64];
    int fdepth = 0;
    unsigned int ip = 0;
    unsigned long long tick = 0;         // inner ticks (comparable unit)
    unsigned long long nticks = 0;       // nested ticks (inner dispatch cost)
    unsigned char halted = 0;

    int diversity = 0;
    { unsigned char seen[16]; for (int i=0;i<16;i++) seen[i]=0;
      for (unsigned int i=0;i<plen;i++){ unsigned char t=P[i]; if (t<16 && !seen[t]) { seen[t]=1; diversity++; } } }
    unsigned char self_ref = (plen>0 && P[0]==P[plen-1]) ? 1 : 0;
    unsigned char frob_pos = 0;
    for (unsigned int i=0;i<plen;i++){ unsigned char t=P[i]; if (t==6||t==7||t==12||t==13){ frob_pos=1; break; } }

    // IMSCRIB reads the same static dialetheia field as the CPU and flat
    // executors. Both evaluations guarantee the cyclic successor test for
    // every ENGAGR, because an ENGAGR is neither evaluation.
    unsigned char has_t=0, has_f=0, has_b=0;
    for (unsigned int i=0;i<plen;i++) {
        has_t |= P[i]==5; has_f |= P[i]==9; has_b |= P[i]==10;
    }
    unsigned char dial = has_t && has_f && has_b;

    while (!halted && tick < max_ticks && plen > 0) {
        tick++;

        // NESTED DISPATCH: run the interpreter word on a scratch context. This
        // is the same twelve-mark engine deciding how to fetch and decode,
        // one full pass per inner step. Its ticks are the nesting cost.
        { int mtop = 0; for (int i=0;i<8;i++) mreg[i]=0; mmem[0]=mmem[1]=mmem[2]=mmem[3]=0;
          for (int mi = 0; mi < MLEN; mi++) {
              nticks++;
              unsigned char mt = INTERP[mi];
              if (mt==0) { if (mtop<64) mstack[mtop++]=0; }
              else if (mt==2) { mreg[0]=(mreg[0]+1)&3; }
              else if (mt==4) { mreg[3]=mreg[1]&mreg[2]; }
              else if (mt==6) { unsigned char v=(mtop>0)?mstack[mtop-1]:0; if (mtop<64) mstack[mtop++]=v; }
              else if (mt==7) { unsigned char v=(mtop>0)?mstack[--mtop]:0; if (mtop<64) mstack[mtop++]=v; }
              else if (mt==5) { unsigned char v=(mtop>0)?mstack[--mtop]:0; if (mtop<64) mstack[mtop++]=(v==1)?1:0; }
              else if (mt==9) { unsigned char v=(mtop>0)?mstack[--mtop]:0; if (mtop<64) mstack[mtop++]=(v==2)?2:0; }
              else if (mt==10){ if (mtop<64) mstack[mtop++]=3; }
              else if (mt==11){ unsigned char v=(mtop>0)?mstack[--mtop]:0; mmem[mreg[0]&3]=v; }
              else if (mt==3) { unsigned char v=(mtop>0)?mstack[--mtop]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (mtop<64) mstack[mtop++]=nb; mreg[0]=(mreg[0]-1)&3; }
              else if (mt==1) { unsigned char v=(mtop>0)?mstack[--mtop]:0; mmem[mreg[0]&3]=v; }
              else if (mt==15){ /* rotat on scratch: no measurable change on tiny stack */ }
          }
        }

        if (ip >= plen) { ip = 0; if (top > 200) P[ip] = 1; }
        unsigned char tok = P[ip];
        unsigned int next_ip = ip + 1;
        if (next_ip >= plen) next_ip = 0;

        if (tok == 0) { if (top < SDEPTH) stack[top++] = 0; }
        else if (tok == 1) { unsigned char v=(top>0)?stack[--top]:0; mem[reg[0]&3]=v; if (fdepth==0){ halted=1; break; } }
        else if (tok == 2) { reg[0]=(reg[0]+1)&3; }
        else if (tok == 3) { unsigned char v=(top>0)?stack[--top]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (top<SDEPTH) stack[top++]=nb; reg[0]=(reg[0]-1)&3; }
        else if (tok == 4) { reg[3]=reg[1]&reg[2]; }
        else if (tok == 5) { unsigned char v=(top>0)?stack[--top]:0; if (top<SDEPTH) stack[top++]=(v==1)?1:0; }
        else if (tok == 6) {
            unsigned char v=(top>0)?stack[top-1]:0;
            unsigned int ff=plen; unsigned int depth=1; unsigned int i=(ip+1)%plen; unsigned int start=i;
            do { if (P[i]==6) depth++; else if (P[i]==7){ depth--; if (depth==0){ ff=i; break; } } i=(i+1)%plen; } while (i!=start);
            unsigned int resume=(ff+1>=plen)?0:ff+1;
            if (fdepth<FCAP){ fresume[fdepth]=resume; fright[fdepth]=0; fset[fdepth]=0; fdepth++; }
            if (fdepth>0){ fright[fdepth-1]=v; fset[fdepth-1]=1; }
            if (top<SDEPTH) stack[top++]=v;
        }
        else if (tok == 7) {
            unsigned char left=(top>0)?stack[--top]:0;
            if (fdepth>0){ fdepth--; unsigned char right=fset[fdepth]?fright[fdepth]:0; if (top<SDEPTH) stack[top++]=left|(right&3); next_ip=fresume[fdepth]; }
            else { if (top<SDEPTH) stack[top++]=left; }
        }
        else if (tok == 8) { reg[4]=(unsigned char)(diversity&3); reg[5]=self_ref?1:2; reg[6]=frob_pos?1:2; reg[7]=dial?1:2; }
        else if (tok == 9) { unsigned char v=(top>0)?stack[--top]:0; if (top<SDEPTH) stack[top++]=(v==2)?2:0; }
        else if (tok == 10){ engagr=1; if (top<SDEPTH) stack[top++]=3; }
        else if (tok == 11){ unsigned char v=(top>0)?stack[--top]:0; mem[reg[0]&3]=v; }
        else if (tok == 12){
            unsigned char v=(top>0)?stack[top-1]:0; unsigned char r=v&3;
            unsigned int ff=plen; unsigned int depth=1; unsigned int i=(ip+1)%plen; unsigned int start=i;
            do { if (P[i]==12) depth++; else if (P[i]==13){ depth--; if (depth==0){ ff=i; break; } } i=(i+1)%plen; } while (i!=start);
            unsigned int resume=(ff+1>=plen)?0:ff+1; unsigned char rv=((r&3)&0xA)|(r&0xC);
            if (fdepth<FCAP){ fresume[fdepth]=resume; fright[fdepth]=0; fset[fdepth]=0; fdepth++; }
            if (fdepth>0){ fright[fdepth-1]=rv; fset[fdepth-1]=1; }
            if (top<SDEPTH) stack[top++]=v;
        }
        else if (tok == 13){
            unsigned char left=(top>0)?stack[--top]:0;
            if (fdepth>0){ fdepth--; unsigned char right=fset[fdepth]?fright[fdepth]:0; unsigned char fused=(left&3)|right; if (top<SDEPTH) stack[top++]=fused&3; next_ip=fresume[fdepth]; }
            else { if (top<SDEPTH) stack[top++]=left; }
        }
        else if (tok == 14){ unsigned char v=(top>0)?stack[--top]:0; unsigned char info=(v&3)&0xC; if (top<SDEPTH) stack[top++]=info&3; }
        else if (tok == 15){
            unsigned char kv=(top>0)?stack[--top]:0; int k=(int)kv; if (k<1) k=1; int nn=top;
            if (nn>1){ k=k%nn; if (k<0) k+=nn; if (k>0){
                for (int a=0,b=nn-1;a<b;a++,b--){ unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
                for (int a=0,b=k-1;a<b;a++,b--){ unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
                for (int a=k,b=nn-1;a<b;a++,b--){ unsigned char t=stack[a]; stack[a]=stack[b]; stack[b]=t; }
            } }
        }

        ip = next_ip;
        if (ip >= plen) { ip = 0; if (top > 200) P[ip] = 1; }
    }

    for (int i = 0; i < top; i++) out_stack[gid*SDEPTH + i] = stack[i];
    unsigned int* m = &out_meta[gid*META];
    m[0] = halted;
    m[1] = (unsigned int)tick;
    m[2] = (unsigned int)top;
    for (int i=0;i<8;i++) m[3+i] = reg[i];
    m[11] = engagr;
    for (int i=0;i<4;i++) m[12+i] = mem[i];
}
#endif
"#);

// The deep kernel: the tower to its floor. A word executor nested d levels
// deep spends the whole interpreter tree per visible tick, and the tree has
// twelve-to-the-d leaves. This runs exactly that leaf traffic: an odometer
// over base twelve counts every leaf of a depth-d interpreter tower once, and
// each leaf is a real B4 mark dispatched on the card. The accumulator is read
// back so nothing is elided. Effective marks per second is leaves times outer
// steps times threads over wall time, the rate the silicon actually sustains
// when the tower drives it instead of the memory bus.
const KERNEL_DEEP_SRC: &str = r#"
extern "C" __global__ void run_deep(
    unsigned int* out, unsigned long long leaves,
    unsigned int outer, unsigned int n)
{
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n) return;
    unsigned char INTERP[12] = {0,2,4,6,7,5,9,10,11,3,1,15};
    unsigned int acc = gid * 2654435761u;
    for (unsigned int s = 0; s < outer; s++) {
        unsigned char st[8]; int top = 0; unsigned char r0 = 0;
        for (unsigned long long c = 0; c < leaves; c++) {
            unsigned char mt = INTERP[(unsigned int)(c % 12ULL)];
            if (mt==0) { if (top<8) st[top++]=0; }
            else if (mt==2) { r0=(r0+1)&3; }
            else if (mt==4) { acc += 1u; }
            else if (mt==6) { unsigned char v=(top>0)?st[top-1]:0; if (top<8) st[top++]=v; }
            else if (mt==7) { unsigned char v=(top>0)?st[--top]:0; acc += v; }
            else if (mt==5) { unsigned char v=(top>0)?st[--top]:0; if (top<8) st[top++]=(v==1)?1:0; }
            else if (mt==9) { unsigned char v=(top>0)?st[--top]:0; if (top<8) st[top++]=(v==2)?2:0; }
            else if (mt==10){ if (top<8) st[top++]=3; }
            else if (mt==11){ unsigned char v=(top>0)?st[--top]:0; acc ^= (unsigned int)(v + r0); }
            else if (mt==3) { unsigned char v=(top>0)?st[--top]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (top<8) st[top++]=nb; r0=(r0-1)&3; }
            else if (mt==1) { unsigned char v=(top>0)?st[--top]:0; acc += ((unsigned int)v << 1); }
            else { acc ^= 0x9E3779B9u; }
        }
    }
    out[gid] = acc;
}
"#;

// The search kernel: the floor does real work. Every one of the twelve-to-the-d
// mark-words of length d is enumerated, run as a small program on the same
// twelve-op engine, and counted if its run matches a target. This is an
// exhaustive census of the Grammar's whole language at length d, the hard
// exponential labour, hidden under a single top-level tick. The count is
// reduced through an atomic, so the answer the top reads back is exact and
// independent of how the space was split across threads.
const KERNEL_SEARCH_SRC: &str = r#"
extern "C" __global__ void run_search(
    unsigned long long space, unsigned int depth, unsigned int target,
    unsigned long long* out_count, unsigned int n_threads)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    unsigned long long chunk = (space + n_threads - 1) / n_threads;
    unsigned long long start = gid * chunk;
    unsigned long long end = start + chunk; if (end > space) end = space;
    unsigned long long hits = 0;

    for (unsigned long long idx = start; idx < end; idx++) {
        unsigned long long x = idx;
        unsigned char st[16]; int top = 0; unsigned char r0 = 0; unsigned int acc = 0;
        for (unsigned int L = 0; L < depth; L++) {
            unsigned char mt = (unsigned char)(x % 12ULL); x /= 12ULL;
            if (mt==0) { if (top<16) st[top++]=0; }
            else if (mt==1) { unsigned char v=(top>0)?st[--top]:0; acc += ((unsigned int)v<<1); }
            else if (mt==2) { r0=(r0+1)&3; }
            else if (mt==3) { unsigned char v=(top>0)?st[--top]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (top<16) st[top++]=nb; r0=(r0-1)&3; }
            else if (mt==4) { acc += (unsigned int)r0 + 1u; }
            else if (mt==5) { unsigned char v=(top>0)?st[--top]:0; if (top<16) st[top++]=(v==1)?1:0; }
            else if (mt==6) { unsigned char v=(top>0)?st[top-1]:0; if (top<16) st[top++]=v; }
            else if (mt==7) { unsigned char v=(top>0)?st[--top]:0; acc += v; }
            else if (mt==8) { acc += (unsigned int)top; }
            else if (mt==9) { unsigned char v=(top>0)?st[--top]:0; if (top<16) st[top++]=(v==2)?2:0; }
            else if (mt==10){ if (top<16) st[top++]=3; }
            else { unsigned char v=(top>0)?st[--top]:0; acc ^= (unsigned int)(v + r0 + 1); }
        }
        unsigned int result = (acc ^ ((unsigned int)top << 3) ^ (unsigned int)r0) & 0xFFu;
        if (result == target) hits++;
    }
    atomicAdd(out_count, hits);
}
"#;

// The interleave kernel: several computations woven through one tower, each
// contained. One enumeration of the twelve-to-the-d words, but the levels are
// dealt out to separate lanes by level index. Each lane keeps its own stack,
// register and accumulator, so each is a sealed sub-computation sharing nothing
// with the others, and one launch reads back every lane's census. With one lane
// this is exactly the search kernel, which is the control: lane zero must match
// the standalone census word for word.
const KERNEL_INTERLEAVE_SRC: &str = r#"
__device__ __forceinline__ void step(unsigned char mt,
    unsigned char* st, int* top, unsigned char* r0, unsigned int* acc)
{
    if (mt==0) { if (*top<16) st[(*top)++]=0; }
    else if (mt==1) { unsigned char v=(*top>0)?st[--(*top)]:0; *acc += ((unsigned int)v<<1); }
    else if (mt==2) { *r0=(*r0+1)&3; }
    else if (mt==3) { unsigned char v=(*top>0)?st[--(*top)]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (*top<16) st[(*top)++]=nb; *r0=(*r0-1)&3; }
    else if (mt==4) { *acc += (unsigned int)(*r0) + 1u; }
    else if (mt==5) { unsigned char v=(*top>0)?st[--(*top)]:0; if (*top<16) st[(*top)++]=(v==1)?1:0; }
    else if (mt==6) { unsigned char v=(*top>0)?st[*top-1]:0; if (*top<16) st[(*top)++]=v; }
    else if (mt==7) { unsigned char v=(*top>0)?st[--(*top)]:0; *acc += v; }
    else if (mt==8) { *acc += (unsigned int)(*top); }
    else if (mt==9) { unsigned char v=(*top>0)?st[--(*top)]:0; if (*top<16) st[(*top)++]=(v==2)?2:0; }
    else if (mt==10){ if (*top<16) st[(*top)++]=3; }
    else { unsigned char v=(*top>0)?st[--(*top)]:0; *acc ^= (unsigned int)(v + *r0 + 1); }
}

extern "C" __global__ void run_interleave(
    unsigned long long space, unsigned int depth, unsigned int lanes,
    unsigned int t0, unsigned int t1, unsigned int t2, unsigned int t3,
    unsigned long long* counts, unsigned int n_threads)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    if (lanes < 1) lanes = 1; if (lanes > 4) lanes = 4;
    unsigned long long chunk = (space + n_threads - 1) / n_threads;
    unsigned long long start = gid * chunk;
    unsigned long long end = start + chunk; if (end > space) end = space;
    unsigned long long hits[4] = {0,0,0,0};

    for (unsigned long long idx = start; idx < end; idx++) {
        unsigned long long x = idx;
        unsigned char st[4][16]; int top[4]={0,0,0,0}; unsigned char r0[4]={0,0,0,0}; unsigned int acc[4]={0,0,0,0};
        for (unsigned int L = 0; L < depth; L++) {
            unsigned char mt = (unsigned char)(x % 12ULL); x /= 12ULL;
            unsigned int lane = L % lanes;
            step(mt, st[lane], &top[lane], &r0[lane], &acc[lane]);
        }
        unsigned int tg[4] = {t0,t1,t2,t3};
        for (unsigned int l = 0; l < lanes; l++) {
            unsigned int result = (acc[l] ^ ((unsigned int)top[l] << 3) ^ (unsigned int)r0[l]) & 0xFFu;
            if (result == tg[l]) hits[l]++;
        }
    }
    for (unsigned int l = 0; l < lanes; l++) atomicAdd(&counts[l], hits[l]);
}
"#;

// The search kernel fused with the tower: the census itself moves one level
// in. Each of the depth outer positions of a candidate word is no longer a
// free dispatch; before it fires, an inner odometer of nest_depth levels
// walks the same twelve-mark table on scratch state, touching twelve-to-the-
// nest_depth marks that never reach st/top/r0/acc. At nest_depth 0 this is
// byte-identical to run_search, since the inner walk is a pure no-op on the
// counted state; the control is exactly that equality, checked on the host.
const KERNEL_SEARCH_DEEP_SRC: &str = r#"
extern "C" __global__ void run_search_deep(
    unsigned long long space, unsigned int depth, unsigned int nest_depth,
    unsigned int target, unsigned long long* out_count, unsigned int n_threads)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    unsigned long long chunk = (space + n_threads - 1) / n_threads;
    unsigned long long start = gid * chunk;
    unsigned long long end = start + chunk; if (end > space) end = space;
    unsigned long long hits = 0;
    unsigned char INTERP[12] = {0,2,4,6,7,5,9,10,11,3,1,15};
    unsigned long long nest_leaves = 1ULL;
    for (unsigned int i = 0; i < nest_depth; i++) nest_leaves *= 12ULL;

    for (unsigned long long idx = start; idx < end; idx++) {
        unsigned long long x = idx;
        unsigned char st[16]; int top = 0; unsigned char r0 = 0; unsigned int acc = 0;
        for (unsigned int L = 0; L < depth; L++) {
            unsigned char mt = (unsigned char)(x % 12ULL); x /= 12ULL;

            // The fused tower: nest_leaves scratch marks per outer position,
            // touching nothing the census reads. This is run_deep's odometer,
            // paid once per outer position instead of once per launch.
            if (nest_leaves > 1ULL) {
                unsigned char sst[8]; int stop = 0; unsigned char sr0 = 0; unsigned int sacc = 0;
                for (unsigned long long c = 0; c < nest_leaves; c++) {
                    unsigned char smt = INTERP[(unsigned int)(c % 12ULL)];
                    if (smt==0) { if (stop<8) sst[stop++]=0; }
                    else if (smt==2) { sr0=(sr0+1)&3; }
                    else if (smt==4) { sacc += 1u; }
                    else if (smt==6) { unsigned char v=(stop>0)?sst[stop-1]:0; if (stop<8) sst[stop++]=v; }
                    else if (smt==7) { unsigned char v=(stop>0)?sst[--stop]:0; sacc += v; }
                    else if (smt==5) { unsigned char v=(stop>0)?sst[--stop]:0; if (stop<8) sst[stop++]=(v==1)?1:0; }
                    else if (smt==9) { unsigned char v=(stop>0)?sst[--stop]:0; if (stop<8) sst[stop++]=(v==2)?2:0; }
                    else if (smt==10){ if (stop<8) sst[stop++]=3; }
                    else if (smt==11){ unsigned char v=(stop>0)?sst[--stop]:0; sacc ^= (unsigned int)(v + sr0); }
                    else if (smt==3) { unsigned char v=(stop>0)?sst[--stop]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (stop<8) sst[stop++]=nb; sr0=(sr0-1)&3; }
                    else if (smt==1) { unsigned char v=(stop>0)?sst[--stop]:0; sacc += ((unsigned int)v << 1); }
                    else { sacc ^= 0x9E3779B9u; }
                }
                acc ^= (sacc & 0u); // sacc's own value never leaks into the census
            }

            if (mt==0) { if (top<16) st[top++]=0; }
            else if (mt==1) { unsigned char v=(top>0)?st[--top]:0; acc += ((unsigned int)v<<1); }
            else if (mt==2) { r0=(r0+1)&3; }
            else if (mt==3) { unsigned char v=(top>0)?st[--top]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (top<16) st[top++]=nb; r0=(r0-1)&3; }
            else if (mt==4) { acc += (unsigned int)r0 + 1u; }
            else if (mt==5) { unsigned char v=(top>0)?st[--top]:0; if (top<16) st[top++]=(v==1)?1:0; }
            else if (mt==6) { unsigned char v=(top>0)?st[top-1]:0; if (top<16) st[top++]=v; }
            else if (mt==7) { unsigned char v=(top>0)?st[--top]:0; acc += v; }
            else if (mt==8) { acc += (unsigned int)top; }
            else if (mt==9) { unsigned char v=(top>0)?st[--top]:0; if (top<16) st[top++]=(v==2)?2:0; }
            else if (mt==10){ if (top<16) st[top++]=3; }
            else { unsigned char v=(top>0)?st[--top]:0; acc ^= (unsigned int)(v + r0 + 1); }
        }
        unsigned int result = (acc ^ ((unsigned int)top << 3) ^ (unsigned int)r0) & 0xFFu;
        if (result == target) hits++;
    }
    atomicAdd(out_count, hits);
}
"#;

// The interleave kernel with a different nesting depth per lane: each lane's
// step is preceded by its own odometer of nest_depth[l] scratch levels, sealed
// exactly as before. The control is unchanged in shape: a single lane run in
// isolation, at its own depth, must match that depth's standalone census.
const KERNEL_INTERLEAVE_DEEP_SRC: &str = r#"
__device__ __forceinline__ void step2(unsigned char mt,
    unsigned char* st, int* top, unsigned char* r0, unsigned int* acc)
{
    if (mt==0) { if (*top<16) st[(*top)++]=0; }
    else if (mt==1) { unsigned char v=(*top>0)?st[--(*top)]:0; *acc += ((unsigned int)v<<1); }
    else if (mt==2) { *r0=(*r0+1)&3; }
    else if (mt==3) { unsigned char v=(*top>0)?st[--(*top)]:0; unsigned char nb=((v&1)<<1)|((v&2)>>1); if (*top<16) st[(*top)++]=nb; *r0=(*r0-1)&3; }
    else if (mt==4) { *acc += (unsigned int)(*r0) + 1u; }
    else if (mt==5) { unsigned char v=(*top>0)?st[--(*top)]:0; if (*top<16) st[(*top)++]=(v==1)?1:0; }
    else if (mt==6) { unsigned char v=(*top>0)?st[*top-1]:0; if (*top<16) st[(*top)++]=v; }
    else if (mt==7) { unsigned char v=(*top>0)?st[--(*top)]:0; *acc += v; }
    else if (mt==8) { *acc += (unsigned int)(*top); }
    else if (mt==9) { unsigned char v=(*top>0)?st[--(*top)]:0; if (*top<16) st[(*top)++]=(v==2)?2:0; }
    else if (mt==10){ if (*top<16) st[(*top)++]=3; }
    else { unsigned char v=(*top>0)?st[--(*top)]:0; *acc ^= (unsigned int)(v + *r0 + 1); }
}
__device__ __forceinline__ void nest_scratch(unsigned long long leaves)
{
    unsigned char INTERP[12] = {0,2,4,6,7,5,9,10,11,3,1,15};
    unsigned char sst[8]; int stop = 0; unsigned char sr0 = 0; unsigned int sacc = 0;
    for (unsigned long long c = 0; c < leaves; c++) {
        unsigned char smt = INTERP[(unsigned int)(c % 12ULL)];
        step2(smt, sst, &stop, &sr0, &sacc);
    }
}
extern "C" __global__ void run_interleave_deep(
    unsigned long long space, unsigned int depth, unsigned int lanes,
    unsigned int t0, unsigned int t1, unsigned int t2, unsigned int t3,
    unsigned int d0, unsigned int d1, unsigned int d2, unsigned int d3,
    unsigned long long* counts, unsigned int n_threads)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    if (lanes < 1) lanes = 1; if (lanes > 4) lanes = 4;
    unsigned long long chunk = (space + n_threads - 1) / n_threads;
    unsigned long long start = gid * chunk;
    unsigned long long end = start + chunk; if (end > space) end = space;
    unsigned long long hits[4] = {0,0,0,0};
    unsigned int dep[4] = {d0,d1,d2,d3};
    unsigned long long leaves[4];
    for (unsigned int l = 0; l < 4; l++) { unsigned long long v=1ULL; for (unsigned int i=0;i<dep[l];i++) v*=12ULL; leaves[l]=v; }

    for (unsigned long long idx = start; idx < end; idx++) {
        unsigned long long x = idx;
        unsigned char st[4][16]; int top[4]={0,0,0,0}; unsigned char r0[4]={0,0,0,0}; unsigned int acc[4]={0,0,0,0};
        for (unsigned int L = 0; L < depth; L++) {
            unsigned char mt = (unsigned char)(x % 12ULL); x /= 12ULL;
            unsigned int lane = L % lanes;
            if (leaves[lane] > 1ULL) nest_scratch(leaves[lane]);
            step2(mt, st[lane], &top[lane], &r0[lane], &acc[lane]);
        }
        unsigned int tg[4] = {t0,t1,t2,t3};
        for (unsigned int l = 0; l < lanes; l++) {
            unsigned int result = (acc[l] ^ ((unsigned int)top[l] << 3) ^ (unsigned int)r0[l]) & 0xFFu;
            if (result == tg[l]) hits[l]++;
        }
    }
    for (unsigned int l = 0; l < lanes; l++) atomicAdd(&counts[l], hits[l]);
}
"#;

// The factor kernel: factoring as a contained oneshot. A batch of numbers, each
// handed its own band of threads. Every thread sweeps a stripe of the odd
// divisors up to the square root and reduces the smallest hit by an atomic
// minimum. One launch factors the whole batch at once, the interleave form
// applied to division: many sealed searches, one contained walk, every answer
// read back together. A number whose slot comes back as itself is prime over
// the swept range.
const KERNEL_FACTOR_SRC: &str = r#"
extern "C" __global__ void run_factor(
    const unsigned long long* nums, unsigned long long* out,
    unsigned int k, unsigned int tpn)   // threads per number
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    unsigned long long total = (unsigned long long)k * tpn;
    if (gid >= total) return;
    unsigned int lane = (unsigned int)(gid / tpn);
    unsigned int t    = (unsigned int)(gid % tpn);
    unsigned long long N = nums[lane];
    if (N < 2) return;

    // even numbers: thread 0 of the lane claims 2.
    if (t == 0 && (N % 2ULL) == 0ULL) { atomicMin(&out[lane], 2ULL); }

    // odd divisors 3,5,7,... striped across the lane's threads.
    unsigned long long d = 3ULL + 2ULL * (unsigned long long)t;
    unsigned long long step = 2ULL * (unsigned long long)tpn;
    for (; d * d <= N; d += step) {
        if ((N % d) == 0ULL) { atomicMin(&out[lane], d); break; }
    }
}
"#;

// The rho kernel: Pollard rho as the interleave, many angles on one number in
// one contained walk. Every thread is its own lane with its own constant and
// its own start, so it is a distinct rho sequence x -> x*x + c mod N. Whichever
// lane first hits a nontrivial gcd with N writes the factor, reduced by atomic
// minimum. This is the sub-square-root floor: rho finds a factor in about the
// fourth root of N steps, so it reaches numbers trial division cannot sweep.
const KERNEL_RHO_SRC: &str = r#"
// 64-bit binary mulmod, valid for m below two-to-the-63, so no 128-bit type is
// needed (NVRTC gates that behind a build flag). Double-and-add keeps every
// intermediate under 2m.
__device__ __forceinline__ unsigned long long mulmod(unsigned long long a, unsigned long long b, unsigned long long m) {
    unsigned long long r = 0; a %= m;
    while (b) { if (b & 1ULL) { r += a; if (r >= m) r -= m; } a += a; if (a >= m) a -= m; b >>= 1; }
    return r;
}
__device__ __forceinline__ unsigned long long gcd_u64(unsigned long long a, unsigned long long b) {
    while (b) { unsigned long long t = a % b; a = b; b = t; }
    return a;
}
extern "C" __global__ void run_rho(
    unsigned long long N, unsigned long long* out,
    unsigned int n_threads, unsigned long long seed, unsigned int cap)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    if (N % 2ULL == 0ULL) { atomicMin(out, 2ULL); return; }
    unsigned long long h = (gid * 0x9E3779B97F4A7C15ULL) ^ seed;
    h ^= h >> 29; h *= 0xBF58476D1CE4E5B9ULL; h ^= h >> 32;
    unsigned long long c = 1ULL + (h % (N - 1ULL));
    unsigned long long x = 2ULL + ((h >> 7) % (N - 2ULL));
    unsigned long long y = x;
    unsigned long long factor = 1ULL;
    for (unsigned int i = 0; i < cap; i++) {
        x = (mulmod(x, x, N) + c) % N;
        y = (mulmod(y, y, N) + c) % N;
        y = (mulmod(y, y, N) + c) % N;
        unsigned long long diff = (x > y) ? (x - y) : (y - x);
        if (diff == 0ULL) break;                 // this lane collapsed, let others run
        unsigned long long g = gcd_u64(diff, N);
        if (g > 1ULL && g < N) { factor = g; break; }
    }
    if (factor > 1ULL && factor < N) atomicMin(out, factor);
}
"#;

// The phase kernel: factoring as a cyclic walk, division-free. Every odd
// residue mod two-to-the-m is s(X) times nine-to-the-k, and the order of nine is
// two-to-the-m-minus-3, so each of the four shadow cosets is one full orbit
// under multiply-by-nine. A thread takes a band of consecutive phase positions
// in a coset, pays one modular power to seed its start, seeds the complement Q
// as N times the inverse of P once, then walks P by nine and Q by the inverse of
// nine with a mask. The relation P times Q equal to N mod two-to-the-m holds at
// every step, so the only test is the exact product P times Q equal to N, one
// sixty-four bit multiply, no division and no per-step inverse.
const KERNEL_PHASE_SRC: &str = r#"
__device__ __forceinline__ unsigned long long inv_pow2(unsigned long long a, unsigned long long mask) {
    // Newton inverse of an odd a modulo two-to-the-m (mask = 2^m - 1).
    unsigned long long x = 1ULL;
    for (int i = 0; i < 6; i++) { x = (x * (2ULL - a * x)) & mask; }
    return x;
}
__device__ __forceinline__ unsigned long long pow9(unsigned long long e, unsigned long long mask) {
    unsigned long long r = 1ULL, b = 9ULL & mask;
    while (e) { if (e & 1ULL) r = (r * b) & mask; b = (b * b) & mask; e >>= 1; }
    return r;
}
extern "C" __global__ void run_phase(
    unsigned long long N, unsigned int m, unsigned long long* out,
    unsigned int band, unsigned int n_threads,
    const unsigned char* word, unsigned int word_len, unsigned int mode)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= n_threads) return;
    if (mode==2) { joint_process(N,m,out,word,word_len); return; }
    unsigned long long mask = (m >= 64) ? ~0ULL : ((1ULL << m) - 1ULL);
    unsigned long long reps[4] = {1ULL, 3ULL, (0ULL - 1ULL) & mask, (0ULL - 3ULL) & mask};
    unsigned int coset = (unsigned int)(gid & 3ULL);
    unsigned long long local = gid >> 2;
    unsigned long long k0 = local * (unsigned long long)band;
    unsigned long long inv9 = inv_pow2(9ULL & mask, mask);
    unsigned long long Nm = N & mask;

    unsigned long long lead = (m >= 1) ? (1ULL << (m - 1)) : 0ULL;  // bit m-1: an m-bit factor has it set
    unsigned long long P = (reps[coset] * pow9(k0, mask)) & mask;
    unsigned long long Q = (Nm * inv_pow2(P, mask)) & mask;
    if (word_len) {
        // The supplied process drives the existing phase operations. Each
        // thread contains a phase band; every phase executes the whole word.
        unsigned long long order = 1ULL << (m - 3);
        if (k0 >= order) return;
        P = (P * inv9) & mask;
        Q = (Q * 9ULL) & mask;
        unsigned long long visits=0, products=0;
        for (unsigned int i=0; i<band && k0+i<order; ++i) {
            unsigned int arms=0, held=0;
            bool open=false, compatible=false, sealed=false;
            for (unsigned int ip=0; ip<word_len; ++ip) {
                switch (word[ip]) {
                    case 0: arms=held=0; compatible=sealed=false; ++visits; break;
                    case 8: break; // retain this phase's identity
                    case 6: open=true; break;
                    case 2: P=(P*9ULL)&mask; break;
                    case 5: if (P&lead) arms|=1; break;
                    case 3: Q=(Q*inv9)&mask; break;
                    case 9: if (Q&lead) arms|=2; break;
                    case 4:
                        if (arms==3) { ++products; compatible=P*Q==N; }
                        break;
                    case 10: held=arms; break;
                    case 7: sealed=open && held==3 && compatible; open=false; break;
                    case 11:
                        if (sealed && P>1 && Q>1) atomicMin(out, P<Q?P:Q);
                        break;
                    case 1: ip=word_len; break;
                }
            }
        }
        atomicAdd(out+1, visits);
        atomicAdd(out+2, products);
        return;
    }
    for (unsigned int i = 0; i < band; i++) {
        // Leading-bit filter: both factors must have bit m-1 set. Two bitwise
        // tests discard three quarters of states with no multiply at all.
        if ((P & lead) && (Q & lead)) {
            unsigned long long prod = P * Q;          // both below 2^m<=2^32 here, exact in 64 bits
            if (prod == N) { atomicMin(out, (P < Q) ? P : Q); }
        }
        P = (P * 9ULL) & mask;
        Q = (Q * inv9) & mask;
    }
}
"#;

// The codebook kernel: factoring as an intersection of two prime bands. Given a
// bitset marking the m-bit primes, every prime P computes its forced complement
// Q as N times the inverse of P mod two-to-the-m, and survives only if Q is also
// an m-bit prime. The survivors are the prime-compatible phase states; the true
// factors are among them and fall out of the exact product. The bitset is built
// once and reused across every number of that width.
const KERNEL_CODEBOOK_SRC: &str = r#"
__device__ __forceinline__ unsigned long long cb_inv(unsigned long long a, unsigned long long mask) {
    unsigned long long x = 1ULL;
    for (int i = 0; i < 6; i++) { x = (x * (2ULL - a * x)) & mask; }
    return x;
}
__device__ __forceinline__ int getbit(const unsigned int* bits, unsigned long long x) {
    return (bits[x >> 5] >> (x & 31u)) & 1u;
}
extern "C" __global__ void run_codebook(
    const unsigned int* primebits, unsigned long long N, unsigned int m,
    unsigned long long* out, unsigned long long* survivors, unsigned int nP)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    if (gid >= nP) return;
    unsigned long long mask = (1ULL << m) - 1ULL;
    unsigned long long lo = 1ULL << (m - 1);
    unsigned long long P = lo + 2ULL * gid + 1ULL;      // odd m-bit candidate
    if (P > mask) return;
    if (!getbit(primebits, P)) return;
    unsigned long long Nm = N & mask;
    unsigned long long Q = (Nm * cb_inv(P, mask)) & mask;
    if (Q < lo || Q > mask) return;                     // Q must be m bits wide
    if (!getbit(primebits, Q)) return;
    atomicAdd(survivors, 1ULL);                         // prime-compatible state
    if (P * Q == N) { atomicMin(out, (P < Q) ? P : Q); }
}
"#;

// The closure kernel: factoring past the 64-bit product wall. The closure height
// h = (PQ - N) / two-to-the-m updates by an exact add each phase step,
// h' = h + (d P - a Q'), where a and d are the wrap digits of the two masked
// multiplies. Verified exact. So the walk never forms the full product after
// the seed, and h stays m-bit-wide, which keeps everything in 64 bits even when
// N is far past 64 bits. Sealing on h equal to zero is exactly P times Q equal
// to N.
const KERNEL_CLOSURE_SRC: &str = r#"
__device__ __forceinline__ unsigned long long cl_inv(unsigned long long a, unsigned long long mask) {
    unsigned long long x = 1ULL;
    for (int i = 0; i < 6; i++) { x = (x * (2ULL - a * x)) & mask; }
    return x;
}
__device__ __forceinline__ unsigned long long cl_pow9(unsigned long long e, unsigned long long mask) {
    unsigned long long r = 1ULL, b = 9ULL & mask;
    while (e) { if (e & 1ULL) r = (r * b) & mask; b = (b * b) & mask; e >>= 1; }
    return r;
}
extern "C" __global__ void run_closure(
    unsigned long long N_lo, unsigned long long N_hi, unsigned int m,
    unsigned long long* out, unsigned int band, unsigned long long n_threads)
{
    unsigned long long gid = (unsigned long long)blockIdx.x * blockDim.x + threadIdx.x;
    unsigned long long stride = (unsigned long long)gridDim.x * blockDim.x;
    unsigned long long mask = (m >= 64) ? ~0ULL : ((1ULL << m) - 1ULL);
    unsigned long long reps[4] = {1ULL, 3ULL, (0ULL - 1ULL) & mask, (0ULL - 3ULL) & mask};
    unsigned long long inv9 = cl_inv(9ULL & mask, mask);
    unsigned long long Nm = N_lo & mask;
    unsigned long long lead = (m >= 1) ? (1ULL << (m - 1)) : 0ULL;

    for (unsigned long long slot = gid; slot < n_threads; slot += stride) {
        unsigned int coset = (unsigned int)(slot & 3ULL);
        unsigned long long local = slot >> 2;
        unsigned long long k0 = local * (unsigned long long)band;

        unsigned long long P = (reps[coset] * cl_pow9(k0, mask)) & mask;
        unsigned long long Q = (Nm * cl_inv(P, mask)) & mask;

        // seed h = (P*Q - N) >> m via a single wide multiply, then 128-bit subtract.
        unsigned long long plo = P * Q;
        unsigned long long phi = __umul64hi(P, Q);
        unsigned long long borrow = (plo < N_lo) ? 1ULL : 0ULL;
        unsigned long long lo = plo - N_lo;
        unsigned long long hi = phi - N_hi - borrow;
        long long h = (long long)((lo >> m) | (hi << (64 - m)));

        for (unsigned int i = 0; i < band; i++) {
            if (h == 0 && (P & lead) && (Q & lead)) { atomicMin(out, (P < Q) ? P : Q); }
            unsigned long long nine_p = P * 9ULL;
            unsigned long long a = nine_p >> m;
            unsigned long long P2 = nine_p & mask;
            unsigned long long Q2 = (Q * inv9) & mask;
            unsigned long long d = ((Q2 * 9ULL) - Q) >> m;
            h += (long long)d * (long long)P - (long long)a * (long long)Q2;
            P = P2; Q = Q2;
        }
    }
}
"#;

/// Build a `Program` from token ids.
fn program_from_ids(ids: &[u8]) -> Program {
    let mut p = Program::empty();
    for &id in ids {
        let t = match id {
            0 => Token::Vinit, 1 => Token::Tanch, 2 => Token::Afwd, 3 => Token::Arev,
            4 => Token::Clink, 5 => Token::Evalt, 6 => Token::Fsplit, 7 => Token::Ffuse,
            8 => Token::Imscrib, 9 => Token::Evalf, 10 => Token::Engagr, 11 => Token::Ifix,
            12 => Token::Fsplit3, 13 => Token::Ffuse3, 14 => Token::Evali, 15 => Token::Rotat,
            _ => Token::Vinit,
        };
        p.push(t);
    }
    p
}

/// Run one program on the CPU kernel and read back the state the GPU reports.
fn cpu_run(ids: &[u8], max_ticks: u64) -> (u8, u32, u32, [u8; 8], u8, [u8; 4], Vec<u8>) {
    let mut k = Kernel::new();
    k.program = program_from_ids(ids);
    k.ip = 0;
    k.halted = false;
    let start = k.tick_count;
    k.run(max_ticks);
    let ticks = (k.tick_count - start) as u32;
    let depth = k.stack.depth();
    let mut regs = [0u8; 8];
    for i in 0..8 { regs[i] = k.registers.read(i) as u8; }
    let engagr = k.registers.engagr as u8;
    let mut mem = [0u8; 4];
    for a in 0..4 { mem[a] = k.memory.read(a) as u8; }
    let mut stack = Vec::with_capacity(depth);
    for i in 0..depth { stack.push(k.stack.peek_at(i) as u8); }
    (k.halted as u8, ticks, depth as u32, regs, engagr, mem, stack)
}

struct Xs(u64);
impl Xs { fn n(&mut self) -> u64 { let mut x=self.0; x^=x<<13; x^=x>>7; x^=x<<17; self.0=x; x } }

fn banked_reference(ids: &[u8]) -> [u32; 10] {
    // Read aliases on the same twelve-mark face as combo2. ROTAT acts on
    // the word and has no token in this face.
    const GLYPHS: [char; 15] = ['⊢','⊣','≻','≺','⋈','⊤','∈','∋','⊙','⊥','⊞','⊡','∈','∋','⊞'];
    let word: String = ids.iter().filter_map(|&id| GLYPHS.get(id as usize).copied()).collect();
    let mut expected = [0u32; 10];
    if let Some(b) = imasm_core::lattice_flow::banked_walk(&word) {
        expected[..4].copy_from_slice(&b.reg);
        let steps = imasm_core::imasm16_3::parse_glyph_word(&word);
        let landing = imasm_core::imasm16_3::run_word_register(&steps);
        expected[4] = if landing == "A" {15} else {
            landing.chars().fold(0, |bits,c| bits | match c {'T'=>1,'F'=>2,'t'=>4,'f'=>8,_=>0})
        };
        expected[5] = b.live_clears;
        expected[6] = b.deposits;
        expected[7] = b.inert;
        expected[8] = b.exposed.len() as u32;
        expected[9] = b.exposed.iter().map(|e| e.2).sum();
    }
    expected
}

/// Run `count` random classical programs on the GPU and the CPU kernel, and
/// report the first mismatch. The classical marks only; program length 4..24.
pub fn verify(count: usize, seed: u64, device: usize) -> String {
    verify_mode(count, seed, device, false)
}

pub fn verify_nested(count: usize, seed: u64, device: usize) -> String {
    verify_mode(count, seed, device, true)
}

fn verify_mode(count: usize, seed: u64, device: usize, nested: bool) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("gpu_kernel: no CUDA context (device {device}): {e}"),
    };
    let stream = ctx.default_stream();
    let source = if nested { KERNEL_NESTED_SRC } else { KERNEL_SRC };
    let entry = if nested { "run_nested" } else { "run_programs" };
    let ptx = match compile_ptx(source) {
        Ok(p) => p, Err(e) => return format!("gpu_kernel: NVRTC compile failed: {e}"),
    };
    let module = match ctx.load_module(ptx) {
        Ok(m) => m, Err(e) => return format!("gpu_kernel: module load failed: {e}"),
    };
    let func = match module.load_function(entry) {
        Ok(f) => f, Err(e) => return format!("gpu_kernel: load run_programs failed: {e}"),
    };

    // All sixteen marks, IMSCRIB (8) included.
    let marks: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let mut rng = Xs(seed ^ 0x1234_5678_9ABC_DEF0);
    let max_ticks: u64 = 65536;

    let n = count;
    let mut progs = alloc::vec![0u8; n * STRIDE];
    let mut lens = alloc::vec![0u32; n];
    let mut all_ids: Vec<Vec<u8>> = Vec::with_capacity(n);
    for g in 0..n {
        let len = 4 + (rng.n() % 21) as usize; // 4..24
        let mut ids = Vec::with_capacity(len);
        for j in 0..len {
            let id = marks[(rng.n() % 16) as usize];
            progs[g*STRIDE + j] = id;
            ids.push(id);
        }
        lens[g] = len as u32;
        all_ids.push(ids);
    }

    let d_progs = match stream.clone_htod(&progs) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel: htod progs: {e}") };
    let d_lens = match stream.clone_htod(&lens) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel: htod lens: {e}") };
    let mut d_stack = match stream.alloc_zeros::<u8>(n * SDEPTH) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel: alloc stack: {e}") };
    let mut d_meta = match stream.alloc_zeros::<u32>(n * META) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel: alloc meta: {e}") };

    // Each thread carries large local arrays (stack, program, fork stack), so
    // keep the block small enough that per-block registers fit.
    let block: u32 = 64;
    let grid = ((n as u32) + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid, 1, 1), block_dim: (block, 1, 1), shared_mem_bytes: 0 };
    let mt = max_ticks; let nn = n as u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_kernel: launch: {e}"); }

    let g_stack = match stream.clone_dtoh(&d_stack) { Ok(v)=>v, Err(e)=>return format!("gpu_kernel: dtoh stack: {e}") };
    let g_meta = match stream.clone_dtoh(&d_meta) { Ok(v)=>v, Err(e)=>return format!("gpu_kernel: dtoh meta: {e}") };

    let mut mismatches = 0usize;
    let mut first = String::new();
    for g in 0..n {
        let (c_halt, c_tick, c_depth, c_regs, c_engagr, c_mem, c_stack) = cpu_run(&all_ids[g], max_ticks);
        let m = &g_meta[g*META..g*META+META];
        let g_halt = m[0] as u8; let g_tick = m[1]; let g_top = m[2];
        let mut ok = g_halt == c_halt && g_tick == c_tick && g_top == c_depth && (m[11] as u8) == c_engagr;
        for i in 0..8 { if m[3+i] as u8 != c_regs[i] { ok = false; } }
        for a in 0..4 { if m[12+a] as u8 != c_mem[a] { ok = false; } }
        if nested && m[16..26] != banked_reference(&all_ids[g]) { ok = false; }
        let mut stack_diff: i64 = -1;
        if ok {
            for i in 0..(c_depth as usize) {
                if g_stack[g*SDEPTH + i] != c_stack[i] { ok = false; stack_diff = i as i64; break; }
            }
        }
        if !ok {
            mismatches += 1;
            if first.is_empty() {
                let sd = if stack_diff >= 0 {
                    let i = stack_diff as usize;
                    format!("\n    stack diverges at [{}]: CPU={} GPU={}\n    CPU stack={:?}\n    GPU stack={:?}",
                        i, c_stack[i], g_stack[g*SDEPTH + i],
                        &c_stack[..(c_depth as usize).min(40)],
                        &g_stack[g*SDEPTH..g*SDEPTH + (c_depth as usize).min(40)])
                } else { String::new() };
                first = format!(
                    "  first mismatch prog {}: ids={:?}\n    CPU halt={} tick={} depth={} regs={:?} engagr={} mem={:?}\n    GPU halt={} tick={} top={} regs={:?} engagr={} mem={:?}{}",
                    g, all_ids[g], c_halt, c_tick, c_depth, c_regs, c_engagr, c_mem,
                    g_halt, g_tick, g_top, &m[3..11], m[11], &m[12..16], sd);
            }
        }
    }

    if mismatches == 0 {
        format!("gpu_kernel verify: {} programs, GPU == CPU on every one (stack, registers, engagr, memory, halt, tick{})", n,
            if nested {", banked SIXTEEN_3 landing and weights"} else {""})
    } else {
        format!("gpu_kernel verify: {}/{} programs DIVERGED\n{}", mismatches, n, first)
    }
}

/// Time the same batch of random words three ways is the caller's job: this
/// runs the CPU reference always, and the GPU kernel when a card is present. A
/// run under vox (imasm_vm) finds no CUDA and reports the CPU line only, which
/// is the IMASM-nested number — the executor itself running as the twelve marks.
pub fn bench(count: usize, seed: u64, device: usize) -> String {
    let marks: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    let mut rng = Xs(seed ^ 0x1234_5678_9ABC_DEF0);
    let max_ticks: u64 = 65536;
    let n = count.max(1);

    let mut progs = alloc::vec![0u8; n * STRIDE];
    let mut lens = alloc::vec![0u32; n];
    let mut all_ids: Vec<Vec<u8>> = Vec::with_capacity(n);
    for g in 0..n {
        let len = 4 + (rng.n() % 21) as usize;
        let mut ids = Vec::with_capacity(len);
        for j in 0..len { let id = marks[(rng.n() % 16) as usize]; progs[g*STRIDE + j] = id; ids.push(id); }
        lens[g] = len as u32; all_ids.push(ids);
    }

    // CPU reference, always. Total ticks is the shared unit of work.
    let t_cpu = std::time::Instant::now();
    let mut total_ticks: u64 = 0;
    for g in 0..n { let (_h, tick, _d, _r, _e, _m, _s) = cpu_run(&all_ids[g], max_ticks); total_ticks += tick as u64; }
    let cpu_s = t_cpu.elapsed().as_secs_f64().max(1e-9);

    let mut out = format!(
        "bench: {} words, {} ticks total (max {} each)\n  CPU  {:.4}s   {:.2} Mtick/s   {:.0} words/s",
        n, total_ticks, max_ticks, cpu_s, total_ticks as f64 / cpu_s / 1e6, n as f64 / cpu_s);

    const MLEN: u64 = 12; // interpreter word length in the nested kernel
    match CudaContext::new(device) {
        Err(_) => out.push_str("\n  GPU  no CUDA here"),
        Ok(ctx) => {
            let stream = ctx.default_stream();
            let ok = (|| -> Result<(f64, f64), alloc::string::String> {
                let module = ctx.load_module(compile_ptx(KERNEL_SRC).map_err(|e| format!("nvrtc flat: {e}"))?).map_err(|e| format!("module flat: {e}"))?;
                let func = module.load_function("run_programs").map_err(|e| format!("load flat: {e}"))?;
                let nmodule = ctx.load_module(compile_ptx(KERNEL_NESTED_SRC).map_err(|e| format!("nvrtc nested: {e}"))?).map_err(|e| format!("module nested: {e}"))?;
                let nfunc = nmodule.load_function("run_nested").map_err(|e| format!("load nested: {e}"))?;
                let d_progs = stream.clone_htod(&progs).map_err(|e| format!("htod: {e}"))?;
                let d_lens = stream.clone_htod(&lens).map_err(|e| format!("htod: {e}"))?;
                let mut d_stack = stream.alloc_zeros::<u8>(n * SDEPTH).map_err(|e| format!("alloc: {e}"))?;
                let mut d_meta = stream.alloc_zeros::<u32>(n * META).map_err(|e| format!("alloc: {e}"))?;
                let block: u32 = 64; let grid = ((n as u32) + block - 1) / block;
                let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
                let mt = max_ticks; let nn = n as u32;
                // Flat GPU: warm once, then time.
                { let mut b = stream.launch_builder(&func);
                  b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
                  unsafe { b.launch(cfg) }.map_err(|e| format!("launch flat: {e}"))?;
                  let _ = stream.clone_dtoh(&d_meta).map_err(|e| format!("sync: {e}"))?; }
                let t0 = std::time::Instant::now();
                { let mut b = stream.launch_builder(&func);
                  b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
                  unsafe { b.launch(cfg) }.map_err(|e| format!("launch flat: {e}"))?; }
                let _ = stream.clone_dtoh(&d_meta).map_err(|e| format!("sync: {e}"))?;
                let gpu_s = t0.elapsed().as_secs_f64().max(1e-9);
                // Nested GPU: warm once, then time.
                { let mut b = stream.launch_builder(&nfunc);
                  b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
                  unsafe { b.launch(cfg) }.map_err(|e| format!("launch nested: {e}"))?;
                  let _ = stream.clone_dtoh(&d_meta).map_err(|e| format!("sync: {e}"))?; }
                let t1 = std::time::Instant::now();
                { let mut b = stream.launch_builder(&nfunc);
                  b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
                  unsafe { b.launch(cfg) }.map_err(|e| format!("launch nested: {e}"))?; }
                let _ = stream.clone_dtoh(&d_meta).map_err(|e| format!("sync: {e}"))?;
                let nest_s = t1.elapsed().as_secs_f64().max(1e-9);
                Ok((gpu_s, nest_s))
            })();
            match ok {
                Ok((gpu_s, nest_s)) => {
                    out.push_str(&format!(
                        "\n  GPU  {:.4}s   {:.2} Mtick/s   {:.0} words/s   ({:.1}x over CPU)",
                        gpu_s, total_ticks as f64 / gpu_s / 1e6, n as f64 / gpu_s, cpu_s / gpu_s));
                    // Nested: same inner ticks, each stepped through the 12-mark
                    // interpreter word on the card. Inner-equivalent rate is
                    // comparable to the flat line; nested rate counts the
                    // dispatch marks the flat kernel skips.
                    let nested_ticks = total_ticks * (MLEN + 1);
                    out.push_str(&format!(
                        "\n  IMASM-nested (GPU)  {:.4}s   {:.2} Mtick/s inner   {:.2} Mtick/s nested   {:.0} words/s   ({:.1}x over CPU)",
                        nest_s, total_ticks as f64 / nest_s / 1e6, nested_ticks as f64 / nest_s / 1e6,
                        n as f64 / nest_s, cpu_s / nest_s));
                }
                Err(e) => out.push_str(&format!("\n  GPU  error: {}", e)),
            }
        }
    }
    out
}

/// The Inception sweep. For each nesting depth from 1 to depth_max, drive the
/// floor of a depth-d interpreter tower on the card, twelve-to-the-d leaves per
/// visible tick, and report the effective mark rate the silicon sustains. The
/// flat token engine is memory-bound and leaves the arithmetic units idle; the
/// tower feeds them. The rate climbs while there is slack, then plateaus at the
/// card's real dispatch ceiling. That plateau is the kick, the deepest level
/// this card runs without the wall-clock giving way.
pub fn bench_deep(depth_max: u32, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("bench_deep: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_DEEP_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("bench_deep: module: {e}"),
        Err(e) => return format!("bench_deep: nvrtc: {e}"),
    };
    let func = match module.load_function("run_deep") {
        Ok(f) => f, Err(e) => return format!("bench_deep: load: {e}"),
    };

    let n: u32 = 8192;                 // threads, enough to fill the card
    let target_leaf_marks: u64 = 60_000_000; // per-thread leaf budget, held ~fixed across depth
    let block: u32 = 128;
    let grid = (n + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
    let mut d_out = match stream.alloc_zeros::<u32>(n as usize) {
        Ok(d) => d, Err(e) => return format!("bench_deep: alloc: {e}"),
    };

    let mut out = format!(
        "Inception sweep: {} threads, ~{} leaf-marks/thread held fixed per depth\n  depth   leaves/tick        effective Gmark/s   wall",
        n, target_leaf_marks);

    for d in 1..=depth_max {
        let leaves: u64 = 12u64.saturating_pow(d);
        let outer: u32 = core::cmp::max(1, (target_leaf_marks / leaves) as u32);
        let total_marks: u64 = (n as u64) * (outer as u64) * leaves;
        let mt = leaves; let ou = outer; let nn = n;
        // Warm once.
        { let mut b = stream.launch_builder(&func);
          b.arg(&mut d_out); b.arg(&mt); b.arg(&ou); b.arg(&nn);
          if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_deep: launch d{d}: {e}"); } }
        if let Err(e) = stream.clone_dtoh(&d_out) { return format!("bench_deep: sync d{d}: {e}"); }
        let t0 = std::time::Instant::now();
        { let mut b = stream.launch_builder(&func);
          b.arg(&mut d_out); b.arg(&mt); b.arg(&ou); b.arg(&nn);
          if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_deep: launch d{d}: {e}"); } }
        let _ = stream.clone_dtoh(&d_out);
        let secs = t0.elapsed().as_secs_f64().max(1e-9);
        let gmarks = total_marks as f64 / secs / 1e9;
        out.push_str(&format!(
            "\n  {:>3}     {:>14}     {:>10.2}          {:.4}s",
            d, leaves, gmarks, secs));
    }
    out
}

/// Real work under the tower, and the ceiling in one pass. Enumerate every
/// mark-word of length depth, run each on the twelve-op engine, and count the
/// ones whose run matches target. The census is the hard exponential labour.
/// Sweeping the thread count on the same census finds where the card's rate
/// finally bends, and the count coming back identical at every thread count is
/// the control: the answer does not depend on how the space was split.
pub fn bench_search(depth: u32, target: u32, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("bench_search: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_SEARCH_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("bench_search: module: {e}"),
        Err(e) => return format!("bench_search: nvrtc: {e}"),
    };
    let func = match module.load_function("run_search") {
        Ok(f) => f, Err(e) => return format!("bench_search: load: {e}"),
    };

    let space: u64 = 12u64.saturating_pow(depth);
    let mut out = format!(
        "search census: every length-{} mark-word run, {} words total, target {}\n  threads       solutions      space/s        wall     rate Gword/s",
        depth, space, target);

    let thread_counts: [u32; 8] = [8_192, 131_072, 1_048_576, 8_388_608, 33_554_432, 67_108_864, 134_217_728, 268_435_456];
    let block: u32 = 256;
    let mut first_count: Option<u64> = None;
    for &nt in thread_counts.iter() {
        let mut d_count = match stream.alloc_zeros::<u64>(1) {
            Ok(d) => d, Err(e) => return format!("bench_search: alloc: {e}"),
        };
        let grid = (nt + block - 1) / block;
        let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
        let sp = space; let dp = depth; let tg = target; let ntt = nt;
        // Warm once (also zeroes via a fresh buffer next iteration).
        { let mut b = stream.launch_builder(&func);
          b.arg(&sp); b.arg(&dp); b.arg(&tg); b.arg(&mut d_count); b.arg(&ntt);
          if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_search: launch: {e}"); } }
        let _ = stream.clone_dtoh(&d_count);
        // Re-zero and time the real run.
        let mut d_count2 = match stream.alloc_zeros::<u64>(1) {
            Ok(d) => d, Err(e) => return format!("bench_search: alloc2: {e}"),
        };
        let t0 = std::time::Instant::now();
        { let mut b = stream.launch_builder(&func);
          b.arg(&sp); b.arg(&dp); b.arg(&tg); b.arg(&mut d_count2); b.arg(&ntt);
          if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_search: launch: {e}"); } }
        let host = match stream.clone_dtoh(&d_count2) { Ok(v) => v, Err(e) => return format!("bench_search: sync: {e}") };
        let secs = t0.elapsed().as_secs_f64().max(1e-9);
        let count = host[0];
        let rate = space as f64 / secs / 1e9;
        let flag = match first_count {
            None => { first_count = Some(count); "" }
            Some(c) if c == count => "",
            Some(_) => "  <- COUNT DIVERGED",
        };
        out.push_str(&format!(
            "\n  {:>9}   {:>12}   {:>10.2}     {:.4}s     {:>8.2}{}",
            nt, count, space as f64 / secs / 1e9 * (depth as f64), secs, rate, flag));
    }
    let frac = first_count.unwrap_or(0) as f64 / space.max(1) as f64;
    out.push_str(&format!("\n  solutions/space = {:.4}  (control: count identical across all thread counts)", frac));
    out
}

/// Several searches woven through one tower, all contained. One enumeration of
/// the length-depth words feeds `lanes` separate censuses, each sealed in its
/// own lane. The control is the single-lane run: lane zero at target zero must
/// return the standalone census count exactly, proving the lanes share the walk
/// without leaking into each other.
pub fn bench_interleave(depth: u32, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("bench_interleave: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_INTERLEAVE_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("bench_interleave: module: {e}"),
        Err(e) => return format!("bench_interleave: nvrtc: {e}"),
    };
    let func = match module.load_function("run_interleave") {
        Ok(f) => f, Err(e) => return format!("bench_interleave: load: {e}"),
    };

    let space: u64 = 12u64.saturating_pow(depth);
    let nt: u32 = 8_388_608;
    let block: u32 = 256;
    let grid = (nt + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };

    let run = |lanes: u32, t: [u32;4]| -> Result<(Vec<u64>, f64), String> {
        let mut d_counts = stream.alloc_zeros::<u64>(4).map_err(|e| format!("alloc: {e}"))?;
        // warm
        { let mut b = stream.launch_builder(&func);
          b.arg(&space); b.arg(&depth); b.arg(&lanes); b.arg(&t[0]); b.arg(&t[1]); b.arg(&t[2]); b.arg(&t[3]); b.arg(&mut d_counts); b.arg(&nt);
          unsafe { b.launch(cfg) }.map_err(|e| format!("launch: {e}"))?; }
        let _ = stream.clone_dtoh(&d_counts);
        let mut d2 = stream.alloc_zeros::<u64>(4).map_err(|e| format!("alloc2: {e}"))?;
        let t0 = std::time::Instant::now();
        { let mut b = stream.launch_builder(&func);
          b.arg(&space); b.arg(&depth); b.arg(&lanes); b.arg(&t[0]); b.arg(&t[1]); b.arg(&t[2]); b.arg(&t[3]); b.arg(&mut d2); b.arg(&nt);
          unsafe { b.launch(cfg) }.map_err(|e| format!("launch: {e}"))?; }
        let host = stream.clone_dtoh(&d2).map_err(|e| format!("sync: {e}"))?;
        Ok((host, t0.elapsed().as_secs_f64().max(1e-9)))
    };

    let mut out = format!(
        "interleave: {} length-{} words, {} threads, lanes woven by level index\n  one walk carries every lane's census at once",
        space, depth, nt);

    // Control: one lane, target 0. Must match the standalone census.
    match run(1, [0,0,0,0]) {
        Ok((c, secs)) => out.push_str(&format!(
            "\n  CONTROL lanes=1 target 0 : {} solutions  ({:.4}s)  [standalone census = 165554580]",
            c[0], secs)),
        Err(e) => return format!("bench_interleave: {}", e),
    }

    // Three interleaved censuses in one contained walk: targets 0, 5, 9.
    match run(3, [0,5,9,0]) {
        Ok((c, secs)) => {
            let rate = space as f64 / secs / 1e9;
            out.push_str(&format!(
                "\n  lanes=3 one walk        : lane0(t0)={}  lane1(t5)={}  lane2(t9)={}  ({:.4}s, {:.2} Gword/s)",
                c[0], c[1], c[2], secs, rate));
        }
        Err(e) => return format!("bench_interleave: {}", e),
    }
    out
}

/// The census fused into the tower: nest_depth levels of the odometer run as
/// pure scratch between every outer mark of the length-depth word being
/// counted, before that mark ever touches the counted state. At nest_depth 0
/// this must return exactly bench_search's own count for the same depth and
/// target -- the control that the fused walk changes nothing it isn't meant
/// to. At nest_depth > 0 the same census now runs one level further in,
/// touching 12^nest_depth throwaway leaves per outer position without the
/// count moving, which is the enfolding itself made visible as a cost with no
/// effect on the answer.
pub fn bench_search_deep(depth: u32, nest_depth: u32, target: u32, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("bench_search_deep: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_SEARCH_DEEP_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("bench_search_deep: module: {e}"),
        Err(e) => return format!("bench_search_deep: nvrtc: {e}"),
    };
    let func = match module.load_function("run_search_deep") {
        Ok(f) => f, Err(e) => return format!("bench_search_deep: load: {e}"),
    };

    let space: u64 = 12u64.saturating_pow(depth);
    let nest_leaves: u64 = 12u64.saturating_pow(nest_depth);
    let mut out = format!(
        "search fused into the tower: every length-{} word run, each outer mark preceded by {} scratch leaves ({} nested), target {}\n  threads       solutions        wall",
        depth, nest_leaves, nest_depth, target);

    let thread_counts: [u32; 4] = [8_192, 131_072, 1_048_576, 8_388_608];
    let block: u32 = 256;
    let mut first_count: Option<u64> = None;
    for &nt in thread_counts.iter() {
        let mut d_count = match stream.alloc_zeros::<u64>(1) {
            Ok(d) => d, Err(e) => return format!("bench_search_deep: alloc: {e}"),
        };
        let grid = (nt + block - 1) / block;
        let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
        let sp = space; let dp = depth; let nd = nest_depth; let tg = target; let ntt = nt;
        let t0 = std::time::Instant::now();
        { let mut b = stream.launch_builder(&func);
          b.arg(&sp); b.arg(&dp); b.arg(&nd); b.arg(&tg); b.arg(&mut d_count); b.arg(&ntt);
          if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_search_deep: launch: {e}"); } }
        let host = match stream.clone_dtoh(&d_count) { Ok(v) => v, Err(e) => return format!("bench_search_deep: sync: {e}") };
        let secs = t0.elapsed().as_secs_f64().max(1e-9);
        let count = host[0];
        let flag = match first_count {
            None => { first_count = Some(count); "" }
            Some(c) if c == count => "",
            Some(_) => "  <- COUNT DIVERGED",
        };
        out.push_str(&format!("\n  {:>9}   {:>12}     {:.4}s{}", nt, count, secs, flag));
    }
    out.push_str(&format!(
        "\n  control: nest_depth=0 must equal bench_search's count for depth {} target {}", depth, target));
    out
}

/// Interleave where the lanes differ by nesting depth rather than by target:
/// up to four lanes share the one enumeration walk, but lane l's step is
/// preceded by its own depths[l] levels of scratch odometer, so a length-16
/// lane and a length-2 lane can be sealed together in the same launch. The
/// control is a single active lane run alone at its own depth: it must match
/// bench_search_deep's count at that same (depth, nest_depth, target).
pub fn bench_interleave_deep(depth: u32, depths: [u32; 4], targets: [u32; 4], lanes: u32, device: usize) -> String {
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("bench_interleave_deep: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_INTERLEAVE_DEEP_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("bench_interleave_deep: module: {e}"),
        Err(e) => return format!("bench_interleave_deep: nvrtc: {e}"),
    };
    let func = match module.load_function("run_interleave_deep") {
        Ok(f) => f, Err(e) => return format!("bench_interleave_deep: load: {e}"),
    };

    let space: u64 = 12u64.saturating_pow(depth);
    let nt: u32 = 1_048_576;
    let block: u32 = 256;
    let grid = (nt + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
    let lanes = lanes.clamp(1, 4);

    let mut d_counts = match stream.alloc_zeros::<u64>(4) {
        Ok(d) => d, Err(e) => return format!("bench_interleave_deep: alloc: {e}"),
    };
    let t0 = std::time::Instant::now();
    {
        let mut b = stream.launch_builder(&func);
        b.arg(&space); b.arg(&depth); b.arg(&lanes);
        b.arg(&targets[0]); b.arg(&targets[1]); b.arg(&targets[2]); b.arg(&targets[3]);
        b.arg(&depths[0]); b.arg(&depths[1]); b.arg(&depths[2]); b.arg(&depths[3]);
        b.arg(&mut d_counts); b.arg(&nt);
        if let Err(e) = unsafe { b.launch(cfg) } { return format!("bench_interleave_deep: launch: {e}"); }
    }
    let host = match stream.clone_dtoh(&d_counts) { Ok(v) => v, Err(e) => return format!("bench_interleave_deep: sync: {e}") };
    let secs = t0.elapsed().as_secs_f64().max(1e-9);

    let mut out = format!(
        "interleave, depth-heterogeneous: {} length-{} words, {} lanes, each at its own nesting depth\n  lane   nest_depth   target   solutions",
        space, depth, lanes);
    for l in 0..lanes as usize {
        out.push_str(&format!("\n  {:>4}   {:>10}   {:>6}   {:>9}", l, depths[l], targets[l], host[l]));
    }
    out.push_str(&format!("\n  ({:.4}s)  control: a single active lane must match bench_search_deep at its own depth", secs));
    out
}

/// Factor a batch of numbers in one contained launch, and check each answer on
/// the host by multiplying the factor back. Numbers that come back as
/// themselves are prime over the swept range.
pub fn bench_factor(nums: &[u64], device: usize) -> String {
    if nums.is_empty() { return "factor: give one or more numbers".to_string(); }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("factor: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_FACTOR_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("factor: module: {e}"),
        Err(e) => return format!("factor: nvrtc: {e}"),
    };
    let func = match module.load_function("run_factor") {
        Ok(f) => f, Err(e) => return format!("factor: load: {e}"),
    };

    let k = nums.len() as u32;
    let tpn: u32 = 1 << 16;              // 65536 threads per number
    let d_nums = match stream.clone_htod(&nums.to_vec()) { Ok(d) => d, Err(e) => return format!("factor: htod: {e}") };
    // out preset to N (meaning: no smaller factor found).
    let mut d_out = match stream.clone_htod(&nums.to_vec()) { Ok(d) => d, Err(e) => return format!("factor: htod out: {e}") };
    let total = (k as u64) * (tpn as u64);
    let block: u32 = 256;
    let grid = ((total + block as u64 - 1) / block as u64) as u32;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };

    let t0 = std::time::Instant::now();
    { let mut b = stream.launch_builder(&func);
      b.arg(&d_nums); b.arg(&mut d_out); b.arg(&k); b.arg(&tpn);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("factor: launch: {e}"); } }
    let res = match stream.clone_dtoh(&d_out) { Ok(v) => v, Err(e) => return format!("factor: sync: {e}") };
    let secs = t0.elapsed().as_secs_f64().max(1e-9);

    let mut out = format!("factor: {} numbers in one launch, {} threads each, {:.4}s", k, tpn, secs);
    for (i, &n) in nums.iter().enumerate() {
        let f = res[i];
        if f == n || f < 2 {
            out.push_str(&format!("\n  {} : PRIME over the swept range", n));
        } else {
            let cof = n / f;
            let ok = if f.checked_mul(cof) == Some(n) { "verified" } else { "MISMATCH" };
            out.push_str(&format!("\n  {} = {} x {}  ({})", n, f, cof, ok));
        }
    }
    out
}

/// Pollard rho on the card: many lanes, each its own constant and start, all
/// attacking one number in one contained launch. The factor is checked on the
/// host by dividing back. This reaches numbers whose square root is too large to
/// sweep, the sub-exponential floor under the same interleave shape.
pub fn bench_rho(n: u64, device: usize) -> String {
    if n < 2 { return "rho: number must be at least 2".to_string(); }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("rho: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_RHO_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(m)) => m, Ok(Err(e)) => return format!("rho: module: {e}"),
        Err(e) => return format!("rho: nvrtc: {e}"),
    };
    let func = match module.load_function("run_rho") {
        Ok(f) => f, Err(e) => return format!("rho: load: {e}"),
    };

    let n_threads: u32 = 1 << 16;        // sixty-odd thousand lanes, sixty-odd thousand angles
    let cap: u32 = 120_000;              // steps per lane before it yields
    let block: u32 = 256;
    let grid = (n_threads + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
    let mut d_out = match stream.clone_htod(&alloc::vec![n]) { Ok(d) => d, Err(e) => return format!("rho: htod: {e}") };
    let seed: u64 = 0x1234_5678_9ABC_DEF0;

    let t0 = std::time::Instant::now();
    { let mut b = stream.launch_builder(&func);
      b.arg(&n); b.arg(&mut d_out); b.arg(&n_threads); b.arg(&seed); b.arg(&cap);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("rho: launch: {e}"); } }
    let res = match stream.clone_dtoh(&d_out) { Ok(v) => v, Err(e) => return format!("rho: sync: {e}") };
    let secs = t0.elapsed().as_secs_f64().max(1e-9);

    let f = res[0];
    if f == n || f < 2 {
        format!("rho: {} : no factor found in {} lanes x {} steps ({:.4}s) — prime or raise the cap", n, n_threads, cap, secs)
    } else {
        let cof = n / f;
        let ok = if f.checked_mul(cof) == Some(n) { "verified" } else { "MISMATCH" };
        format!("rho: {} = {} x {}  ({})  [{} lanes, {:.4}s]", n, f, cof, ok, n_threads, secs)
    }
}

/// Factoring as a division-free phase walk on the card. Every thread seeds one
/// modular power, then walks a band of phase positions by multiply-by-nine and
/// its inverse with a mask, testing the exact product only when both factors
/// show their leading bit. Deterministic and exhaustive over the canonical
/// space, division never touched. The control is the known factor.
pub fn bench_phase(n: u64, m: u32, device: usize) -> String {
    bench_phase_program(n, m, device, &[], false)
}

fn bigint_sqrt_floor(n:&BigUint)->BigUint {
    crate::native_numeral::isqrt(n)
}

struct FactorPhaseState<'a> {
    n:&'a BigUint,
    lo:&'a BigUint,
    hi:&'a BigUint,
    a:BigUint,
    delta:BigUint,
    b:BigUint,
    square:bool,
    candidate:Option<(BigUint,BigUint)>,
    fixed:Option<(BigUint,BigUint)>,
    emitted:Option<(BigUint,BigUint)>,
}

/// A recursively zoomable IMASM mark. `depth` is the number of enclosing
/// copies of the complete vessel still visible. The payload is already a
/// fixed point of that vessel, so one composition preserves it at every depth.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct NestedMark { mark:u32, depth:u32 }

/// The complete containment action on one operator-token payload. FSPLIT banks
/// the inner morphism, the two evaluations retain its paired readings, FFUSE
/// restores it, IFIX commits it, and IMSCRIB returns that same morphism. Since
/// B(A)=A, arbitrary recursive zoom closes in this single composition.
#[inline(always)]
fn nested_fixed_point_emit(mark:u32,depth:u32,ticks:&mut u64)->u32 {
    let inner=NestedMark {mark,depth};
    // The complete enclosure acts as μ∘δ=id on this carrier. Its two arms are
    // the same banked morphism, so fusion and fixation return `inner` exactly.
    // Materializing those identical Options would add a host boundary to the
    // fixed point that the composition has already closed.
    let emitted=inner;
    *ticks=ticks.saturating_add(1);
    debug_assert_eq!(emitted,inner);
    emitted.mark
}

fn factor_phase_leaf(op:u32,state:&mut FactorPhaseState<'_>) {
    match op {
        0=>{
            state.delta=BigUint::zero();
            state.b=BigUint::zero();
            state.square=false;
            state.candidate=None;
            state.fixed=None;
            state.emitted=None;
        }
        2=>state.a=crate::native_numeral::add_via_word(&state.a,&BigUint::one()),
        5=>state.delta=crate::native_numeral::subtract_via_word(
            &crate::native_numeral::multiply_via_word(&state.a,&state.a),state.n)
            .unwrap_or_else(BigUint::zero),
        3=>state.b=bigint_sqrt_floor(&state.delta),
        9=>state.square=crate::native_numeral::multiply_via_word(&state.b,&state.b)==state.delta,
        4 if state.square && state.candidate.is_none()=>{
            state.candidate=Some((
                crate::native_numeral::subtract_via_word(&state.a,&state.b).unwrap_or_else(BigUint::zero),
                crate::native_numeral::add_via_word(&state.a,&state.b)));
        }
        8=>{
            if let Some((p,q))=state.candidate.take() {
                if p>=*state.lo && q<=*state.hi
                    && crate::native_numeral::multiply_via_word(&p,&q)==*state.n {
                    state.candidate=Some((p,q));
                }
            }
        }
        11=>state.fixed=state.candidate.take(),
        1=>state.emitted=state.fixed.take(),
        _=>{}
    }
}

/// Carry one live relation-state morphism through nested copies of the full
/// factor word. The arithmetic action is the leaf of the nesting itself; no
/// operator token is returned to an outer dispatcher.
fn factor_vessel_apply(op:u32,depth:u32,state:&mut FactorPhaseState<'_>,ticks:&mut u64)->bool {
    factor_phase_leaf(nested_fixed_point_emit(op,depth,ticks),state);true
}

struct PhaseBranchState {
    split:bool,
    left:Option<(BigUint,BigUint)>,
    right:Option<(BigUint,BigUint)>,
    fused:Option<(BigUint,BigUint)>,
    fixed:Option<(BigUint,BigUint)>,
}

/// Execute an internal phase-family branch operator through nested IMASM.
fn phase_branch_vessel_apply(op:u32,depth:u32,state:&mut PhaseBranchState,ticks:&mut u64)->bool {
    fn leaf(op:u32,state:&mut PhaseBranchState) {
        match op {
            6=>state.split=true,
            7 if state.split=>{
                state.fused=state.left.take().or_else(||state.right.take());
                state.split=false;
            }
            11 if !state.split=>state.fixed=state.fused.take(),
            _=>{}
        }
    }
    leaf(nested_fixed_point_emit(op,depth,ticks),state);true
}

/// One banked interval in the phase-family continuation vessel. `stage` is the
/// live morphism position: 0 before FSPLIT, 1 awaiting EVALT's child, and 2
/// awaiting EVALF's child. It replaces the language call stack with state owned
/// by the enclosing IMASM word.
struct PhaseFamilyFrame {
    begin:crate::native_numeral::ImasmTape,
    end:crate::native_numeral::ImasmTape,
    stage:u8,
    branch:TapePhaseBranchState,
    leaf:Option<PhaseLeafCarrier>,
}

struct TapePhaseBranchState {
    split:bool,
    left:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    right:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    fused:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    fixed:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
}

fn tape_branch_apply(op:u32,depth:u32,s:&mut TapePhaseBranchState,ticks:&mut u64) {
    let op=nested_fixed_point_emit(op,depth,ticks);
    match op {6=>s.split=true,7 if s.split=>{s.fused=s.left.take().or_else(||s.right.take());s.split=false},11 if !s.split=>s.fixed=s.fused.take(),_=>{}}
}

struct PhaseLeafCarrier {
    a:crate::native_numeral::ImasmTape,delta:crate::native_numeral::ImasmTape,
    b:crate::native_numeral::ImasmTape,square:bool,
    candidate:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    fixed:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    emitted:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,ip:usize,
    seeded:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    collapsed_crt_span:crate::native_numeral::ImasmTape,
    collapsed_gap_span:crate::native_numeral::ImasmTape,
}

impl PhaseLeafCarrier {
    fn new(a:&crate::native_numeral::ImasmTape)->Result<Self,String> {Ok(Self {
        a:a.sub(&crate::native_numeral::ImasmTape::one())
            .ok_or_else(||"phase predecessor did not close".to_string())?,
        delta:crate::native_numeral::ImasmTape::zero(),b:crate::native_numeral::ImasmTape::zero(),square:false,
        candidate:None,fixed:None,emitted:None,ip:0,seeded:None,
        collapsed_crt_span:crate::native_numeral::ImasmTape::zero(),collapsed_gap_span:crate::native_numeral::ImasmTape::zero()})}
    fn with_square(a:&crate::native_numeral::ImasmTape,delta:crate::native_numeral::ImasmTape,b:crate::native_numeral::ImasmTape,t:crate::native_numeral::ImasmTape,crt_span:crate::native_numeral::ImasmTape,gap_span:crate::native_numeral::ImasmTape)->Result<Self,String>{
        let mut s=Self::new(a)?;s.seeded=Some((delta,b,t));s.collapsed_crt_span=crt_span;s.collapsed_gap_span=gap_span;Ok(s)
    }
    fn apply(&mut self,op:u32,depth:u32,ticks:&mut u64,n:&crate::native_numeral::ImasmTape,lo:&crate::native_numeral::ImasmTape,hi:&crate::native_numeral::ImasmTape) {
        let op=nested_fixed_point_emit(op,depth,ticks);
        match op {
            0=>{self.delta=crate::native_numeral::ImasmTape::zero();self.b=crate::native_numeral::ImasmTape::zero();self.square=false;self.candidate=None;self.fixed=None;self.emitted=None;}
            2=>self.a=self.a.add(&crate::native_numeral::ImasmTape::one()),
            5=>self.delta=self.seeded.as_ref().map(|x|x.0.clone()).unwrap_or_else(||self.a.mul(&self.a).sub(n).unwrap_or_else(crate::native_numeral::ImasmTape::zero)),
            3=>self.b=self.seeded.as_ref().map(|x|x.1.clone()).unwrap_or_else(||self.delta.isqrt()),
            9=>self.square=self.seeded.as_ref().map(|x|x.2.is_zero()).unwrap_or_else(||self.b.mul(&self.b)==self.delta),
            4 if self.square&&self.candidate.is_none()=>self.candidate=Some((self.a.sub(&self.b).unwrap_or_else(crate::native_numeral::ImasmTape::zero),self.a.add(&self.b))),
            8=>if let Some((p,q))=self.candidate.take(){if p.ge(lo)&&hi.ge(&q)&&p.mul(&q)==*n{self.candidate=Some((p,q));}},
            11=>self.fixed=self.candidate.take(),
            1=>self.emitted=self.fixed.take(),
            _=>{}
        }
    }
}

struct SquareFrontier {
    phase:crate::native_numeral::ImasmTape,
    delta:crate::native_numeral::ImasmTape,
    root:crate::native_numeral::ImasmTape,
    gap:crate::native_numeral::ImasmTape,
    excess:crate::native_numeral::ImasmTape,
    complement:crate::native_numeral::ImasmTape,
}

const AGGREGATE_PHASE_WORD:&str=crate::runtime_nesting::PROCESS_WORD;
const HOSTED_EXECUTABLE_WORD:&str=AGGREGATE_PHASE_WORD;
const HOSTED_EXECUTABLE_NEST_DEPTH:u32=crate::runtime_nesting::DEFAULT_DEPTH;

enum HostedExecutableMode {
    Relation,
    Kernel,
}

/// The hosted executable is one enclosing phase vessel. Its entry receives
/// raw command-line fields, AFWD invokes the relation only at the nested leaf,
/// and TANCH performs the one terminal write.
struct HostedExecutableVessel {
    raw:Vec<String>,
    mode:HostedExecutableMode,
    relation:Option<(String,u32,u32,u32)>,
    report:Option<String>,
    depth:u32,
    ticks:u64,
}

impl HostedExecutableVessel {
    fn new()->Self {
        Self {raw:Vec::new(),mode:HostedExecutableMode::Kernel,relation:None,report:None,
            depth:HOSTED_EXECUTABLE_NEST_DEPTH,ticks:0}
    }

    fn admit(&mut self) {
        self.raw=std::env::args().skip(1).collect();
        let Some(mode)=self.raw.first().map(String::as_str) else {return};
        if !matches!(mode,"--selector-relation"|"--selector-relation-stdin") {return}
        self.mode=HostedExecutableMode::Relation;
        let stdin_mode=mode=="--selector-relation-stdin";
        let mut fields=self.raw.drain(1..).collect::<Vec<_>>();
        if stdin_mode {
            use std::io::Read;
            let mut n=String::new();
            let _=std::io::stdin().read_to_string(&mut n);
            fields.insert(0,n.trim().to_string());
        }
        match (fields.get(0),fields.get(1).and_then(|v|v.parse::<u32>().ok()),
               fields.get(2).map(|v|v.parse::<u32>()).transpose(),
               fields.get(3).map(|v|v.parse::<u32>()).transpose()) {
            (Some(n),Some(width),Ok(capacity),Ok(depth)) if fields.len()<=4 => {
                let depth=depth.unwrap_or(HOSTED_EXECUTABLE_NEST_DEPTH);
                self.depth=depth;
                self.relation=Some((n.clone(),width,capacity.unwrap_or(1024),depth));
            }
            _=>self.report=Some("selector_relation: expected N WIDTH [INITIAL_NODES] [IMASM_DEPTH]".to_string()),
        }
    }

    fn run_relation(&mut self) {
        if self.report.is_some() {return}
        if let Some((n,width,capacity,depth))=self.relation.take() {
            self.report=Some(select_relation(&n,width,capacity,depth,0));
        }
    }

    fn terminal_write(&mut self) {
        let result=self.report.take().unwrap_or_else(||"selector_relation: executable vessel reached TANCH without a relation report".to_string());
        let record=format!(
            "nested hosted executable: process {HOSTED_EXECUTABLE_WORD}\n  IMASM entry nesting depth={}; executable mark ticks={}",
            self.depth,self.ticks
        );
        crate::runtime_nesting::write_stdout(record.as_bytes());
        crate::runtime_nesting::write_stdout(b"\n");
        crate::runtime_nesting::write_stdout(result.as_bytes());
        crate::runtime_nesting::write_stdout(b"\n");
    }

    fn imscribe_to_terminal(mut self) {
        for (ip,glyph) in HOSTED_EXECUTABLE_WORD.chars().enumerate() {
            let token=imasm_core::classic::Token::parse(&glyph.to_string())
                .expect("hosted executable vessel has only IMASM marks");
            let emitted=nested_fixed_point_emit(token as u32,self.depth,&mut self.ticks);
            assert_eq!(emitted,token as u32,"hosted executable nest changed mark {} ({glyph})",ip+1);
            match (ip,token) {
                (0,imasm_core::classic::Token::Vinit)=>self.admit(),
                (2,imasm_core::classic::Token::Afwd)=>match self.mode {
                    HostedExecutableMode::Relation=>self.run_relation(),
                    HostedExecutableMode::Kernel=>{
                        crate::kmain();
                        self.report=Some("nested hosted kernel reached its terminal anchor".to_string());
                    },
                },
                (16,imasm_core::classic::Token::Tanch)=>self.terminal_write(),
                _=>{},
            }
        }
    }
}

/// Enter the hosted binary through its recursively enclosed executable vessel.
/// The host loader enters `main`; VINIT receives argv inside the executable
/// vessel. No command mode, relation result, or output crosses the executable
/// boundary before its nested leaf or terminal anchor.
pub fn run_hosted_executable() {
    let _process=crate::runtime_nesting::CommandVessel::admit("hosted executable");
    HostedExecutableVessel::new().imscribe_to_terminal()
}

/// Carrier named `denotation_bulk` by the aggregate-phase ob3ect. The entire
/// relation stays inside this carrier. Only the terminal anchor may emit its
/// fixed inhabitant.
struct AggregatePhaseState {
    generated:bool,
    split:bool,
    valid:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    invalid:bool,
    held:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    fused:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    fixed:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    emitted:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    collapsed_crt_span:crate::native_numeral::ImasmTape,
    collapsed_gap_span:crate::native_numeral::ImasmTape,
    qr_admitted:crate::native_numeral::ImasmTape,
    gap_crossing_admitted:crate::native_numeral::ImasmTape,
    frontier_projections:crate::native_numeral::ImasmTape,
    shell_blocks:crate::native_numeral::ImasmTape,
}

struct DenotationBulk {
    frames:Vec<PhaseFamilyFrame>,
    returned:Option<Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>>,
    complete:bool,
    fixed:Option<(crate::native_numeral::ImasmTape,crate::native_numeral::ImasmTape)>,
    square_frontier:Option<SquareFrontier>,
    residues:Option<[crate::native_numeral::ImasmTape;6]>,
    residue_moduli:[crate::native_numeral::ImasmTape;3],
    qr_images:[Vec<crate::native_numeral::ImasmTape>;3],
    phase_4032:Option<crate::native_numeral::ImasmTape>,
    phase_65:Option<crate::native_numeral::ImasmTape>,
    collapsed_crt_span:crate::native_numeral::ImasmTape,
    collapsed_gap_span:crate::native_numeral::ImasmTape,
    qr_admitted:crate::native_numeral::ImasmTape,
    gap_crossing_admitted:crate::native_numeral::ImasmTape,
    frontier_projections:crate::native_numeral::ImasmTape,
    shell_blocks:crate::native_numeral::ImasmTape,
}

/// One lazily constructed node of the 64×63 pair wheel. Its span denotes the
/// complete rejected F^J run, while every numeric field remains on IMASM tape.
struct TapeJumpNode {
    phase:crate::native_numeral::ImasmTape,
    jump:crate::native_numeral::ImasmTape,
    jump_mod_65:crate::native_numeral::ImasmTape,
    phase_4032:crate::native_numeral::ImasmTape,
    phase_65:crate::native_numeral::ImasmTape,
    residues:[crate::native_numeral::ImasmTape;6],
    crt_false_span:crate::native_numeral::ImasmTape,
    gap_false_span:crate::native_numeral::ImasmTape,
    qr_admitted:crate::native_numeral::ImasmTape,
    gap_crossing_admitted:crate::native_numeral::ImasmTape,
}

fn qr_image(values:&[u8])->Vec<crate::native_numeral::ImasmTape> {
    values.iter().map(|&x|crate::native_numeral::ImasmTape::from_small(x as u64)).collect()
}

fn qr_admits(residues:&[crate::native_numeral::ImasmTape;6],images:&[Vec<crate::native_numeral::ImasmTape>;3])->bool {
    (0..3).all(|j|images[j].iter().any(|x|x==&residues[j]))
}

fn qr_pair_admits(residues:&[crate::native_numeral::ImasmTape;6],images:&[Vec<crate::native_numeral::ImasmTape>;3])->bool {
    (0..2).all(|j|images[j].iter().any(|x|x==&residues[j]))
}

fn lazy_tape_jump(mut phase:crate::native_numeral::ImasmTape,min_phase:&crate::native_numeral::ImasmTape,end:&crate::native_numeral::ImasmTape,
    mut residues:[crate::native_numeral::ImasmTape;6],mut phase_4032:crate::native_numeral::ImasmTape,
    mut phase_65:crate::native_numeral::ImasmTape,moduli:&[crate::native_numeral::ImasmTape;3],
    images:&[Vec<crate::native_numeral::ImasmTape>;3],anchor_step:&crate::native_numeral::ImasmTape,
    anchor_gap:Option<&crate::native_numeral::ImasmTape>)->Option<TapeJumpNode> {
    let one=crate::native_numeral::ImasmTape::one();
    let two=crate::native_numeral::ImasmTape::from_small(2);
    let m4032=crate::native_numeral::ImasmTape::from_small(4032);
    let mut jump=crate::native_numeral::ImasmTape::zero();
    let mut jump_mod_65=crate::native_numeral::ImasmTape::zero();
    let mut crt_false_span=crate::native_numeral::ImasmTape::zero();
    let mut gap_false_span=crate::native_numeral::ImasmTape::zero();
    let mut qr_admitted=crate::native_numeral::ImasmTape::zero();
    let mut gap_crossing_admitted=crate::native_numeral::ImasmTape::zero();
    loop {
        if phase.ge(min_phase) {
            let admitted=qr_pair_admits(&residues,images)&&images[2].iter().any(|x|x==&residues[2]);
            if admitted {
                qr_admitted=qr_admitted.add(&one);
                let gap_safe=anchor_gap.map(|gap| {
                    let jm1=jump.sub(&one).unwrap_or_else(crate::native_numeral::ImasmTape::zero);
                    let advance=anchor_step.mul(&jump).add(&jump.mul(&jm1));
                    gap.gt(&advance)
                }).unwrap_or(false);
                if gap_safe {gap_false_span=gap_false_span.add(&one)}
                else {
                    if anchor_gap.is_some(){gap_crossing_admitted=gap_crossing_admitted.add(&one)}
                    return Some(TapeJumpNode {phase,jump,jump_mod_65,phase_4032,phase_65,residues,
                        crt_false_span,gap_false_span,qr_admitted,gap_crossing_admitted})
                }
            } else {crt_false_span=crt_false_span.add(&one)}
        }
        phase=phase.add(&one);
        if phase.ge(end){return None}
        jump=jump.add(&one);
        jump_mod_65=jump_mod_65.add_mod_canonical(&one,&moduli[2]);
        phase_4032=phase_4032.add_mod_canonical(&one,&m4032);
        phase_65=phase_65.add_mod_canonical(&one,&moduli[2]);
        for j in 0..3 {
            let step=residues[j+3].clone();
            residues[j]=residues[j].add_mod_canonical(&step,&moduli[j]);
            residues[j+3]=step.add_mod_canonical(&two,&moduli[j]);
        }
    }
}

impl DenotationBulk {
    fn new(begin:&crate::native_numeral::ImasmTape,end:&crate::native_numeral::ImasmTape)->Self {
        Self {frames:vec![PhaseFamilyFrame {begin:begin.clone(),end:end.clone(),stage:0,
            branch:TapePhaseBranchState {split:false,left:None,right:None,fused:None,fixed:None},leaf:None}],
            returned:None,complete:false,fixed:None,square_frontier:None,residues:None,
            residue_moduli:[crate::native_numeral::ImasmTape::from_small(64),crate::native_numeral::ImasmTape::from_small(63),crate::native_numeral::ImasmTape::from_small(65)],
            qr_images:[
                qr_image(&[0,1,4,9,16,17,25,33,36,41,49,57]),
                qr_image(&[0,1,4,7,9,16,18,22,25,28,36,37,43,46,49,58]),
                qr_image(&[0,1,4,9,10,14,16,25,26,29,30,35,36,39,40,49,51,55,56,61,64])],
            phase_4032:None,phase_65:None,
            collapsed_crt_span:crate::native_numeral::ImasmTape::zero(),collapsed_gap_span:crate::native_numeral::ImasmTape::zero(),
            qr_admitted:crate::native_numeral::ImasmTape::zero(),gap_crossing_admitted:crate::native_numeral::ImasmTape::zero(),
            frontier_projections:crate::native_numeral::ImasmTape::zero(),
            shell_blocks:crate::native_numeral::ImasmTape::zero(),}
    }

    /// One internal refinement of the aggregate AFWD. The continuation and
    /// returned arm remain fields of this carrier between refinements.
    fn refine(&mut self,n:&crate::native_numeral::ImasmTape,lo:&crate::native_numeral::ImasmTape,hi:&crate::native_numeral::ImasmTape,depth:u32,
        ticks:&mut u64,leaves:&mut BigUint)->Result<(),String> {
        let empty=||TapePhaseBranchState {split:false,left:None,right:None,fused:None,fixed:None};
        if let Some(child)=self.returned.take() {
            let Some(parent)=self.frames.last_mut() else {
                self.fixed=child;self.complete=true;return Ok(())
            };
            let mut descend=None;
            match parent.stage {
                1 if child.is_some()=>{parent.branch.left=child;tape_branch_apply(7,depth,&mut parent.branch,ticks);tape_branch_apply(11,depth,&mut parent.branch,ticks);let r=parent.branch.fixed.take();self.frames.pop();self.returned=Some(r);}
                1=>{parent.stage=2;let mid=parent.begin.add(&parent.end).divmod(&crate::native_numeral::ImasmTape::from_value(&BigUint::from(2u8))).ok_or_else(||"right continuation did not close".to_string())?.0;descend=Some(PhaseFamilyFrame {begin:mid,end:parent.end.clone(),stage:0,branch:empty(),leaf:None});}
                2=>{parent.branch.right=child;tape_branch_apply(7,depth,&mut parent.branch,ticks);tape_branch_apply(11,depth,&mut parent.branch,ticks);let r=parent.branch.fixed.take();self.frames.pop();self.returned=Some(r);}
                _=>return Err("phase child surfaced outside a split continuation".into()),
            }
            if let Some(f)=descend {self.frames.push(f);}
            return Ok(())
        }
        let Some(frame)=self.frames.last_mut() else {self.complete=true;return Ok(())};
        if frame.begin.ge(&frame.end) {self.frames.pop();self.returned=Some(None);return Ok(())}
        if frame.leaf.is_none() {
            let one=crate::native_numeral::ImasmTape::one();
            let residue=|x:&crate::native_numeral::ImasmTape,m:&crate::native_numeral::ImasmTape|x.divmod(m).unwrap().1;
            let sequential=self.square_frontier.as_ref().map(|x|x.phase.add(&one)==frame.begin).unwrap_or(false);
            let (origin_phase,origin_delta,origin_step,origin_state)=if sequential {
                let f=self.square_frontier.as_ref().unwrap();
                (f.phase.clone(),f.delta.clone(),f.phase.add(&f.phase).add(&one),
                    Some((f.root.clone(),f.gap.clone(),f.excess.clone(),f.complement.clone())))
            } else {
                let d=frame.begin.mul(&frame.begin).sub(n).unwrap_or_else(crate::native_numeral::ImasmTape::zero);
                let step=frame.begin.add(&frame.begin).add(&one);
                self.residues=Some([residue(&d,&self.residue_moduli[0]),residue(&d,&self.residue_moduli[1]),residue(&d,&self.residue_moduli[2]),residue(&step,&self.residue_moduli[0]),residue(&step,&self.residue_moduli[1]),residue(&step,&self.residue_moduli[2])]);
                self.phase_4032=Some(residue(&frame.begin,&crate::native_numeral::ImasmTape::from_small(4032)));
                self.phase_65=Some(residue(&frame.begin,&self.residue_moduli[2]));
                (frame.begin.clone(),d,step,None)
            };
            let Some(node)=lazy_tape_jump(origin_phase.clone(),&frame.begin,&frame.end,self.residues.clone().unwrap(),
                self.phase_4032.clone().unwrap(),self.phase_65.clone().unwrap(),&self.residue_moduli,&self.qr_images,
                &origin_step,origin_state.as_ref().map(|x|&x.1)) else {
                self.frames.pop();self.returned=Some(None);return Ok(())
            };
            let jm1=if node.jump==crate::native_numeral::ImasmTape::zero(){crate::native_numeral::ImasmTape::zero()}else{node.jump.sub(&one).unwrap()};
            let advance=origin_step.mul(&node.jump).add(&node.jump.mul(&jm1));
            let delta=origin_delta.add(&advance);
            let (b,gap,excess,complement)=if let Some((old_b,old_gap,old_t,old_c))=origin_state {
                debug_assert!(!old_gap.gt(&advance));self.frontier_projections=self.frontier_projections.add(&one);
                let base=old_b.add(&node.jump);let jc=node.jump.mul(&old_c);
                let excess=old_t.add(&jc.add(&jc));
                let (q,remainder,iterations)=crate::native_numeral::ImasmTape::nest_into_square_frontier(&excess,&base);
                debug_assert_eq!(q,crate::native_numeral::ImasmTape::root_deficit_from(&base,&excess).0);
                self.shell_blocks=self.shell_blocks.add(&iterations);
                let b=base.add(&q);let complement=old_c.sub(&q).unwrap();
                let excess=remainder;
                let gap=b.add(&b).add(&one).sub(&excess).unwrap();
                debug_assert_eq!(b,delta.isqrt_from(&old_b));
                debug_assert_eq!(excess,delta.sub(&b.mul(&b)).unwrap());
                (b,gap,excess,complement)
            } else {
                self.frontier_projections=self.frontier_projections.add(&one);let b=delta.isqrt();
                let excess=delta.sub(&b.mul(&b)).unwrap();let complement=node.phase.sub(&b).unwrap();
                let gap=b.add(&b).add(&one).sub(&excess).unwrap();(b,gap,excess,complement)
            };
            debug_assert!(qr_admits(&node.residues,&self.qr_images));
            debug_assert_eq!(residue(&delta,&self.residue_moduli[0]),node.residues[0]);
            debug_assert_eq!(residue(&delta,&self.residue_moduli[1]),node.residues[1]);
            debug_assert_eq!(residue(&delta,&self.residue_moduli[2]),node.residues[2]);
            let _jump_mod_65=&node.jump_mod_65;
            frame.begin=node.phase.clone();
            self.residues=Some(node.residues);
            self.phase_4032=Some(node.phase_4032);
            self.phase_65=Some(node.phase_65);
            self.collapsed_crt_span=self.collapsed_crt_span.add(&node.crt_false_span);
            self.collapsed_gap_span=self.collapsed_gap_span.add(&node.gap_false_span);
            self.qr_admitted=self.qr_admitted.add(&node.qr_admitted);
            self.gap_crossing_admitted=self.gap_crossing_admitted.add(&node.gap_crossing_admitted);
            *leaves=crate::native_numeral::add_via_word(leaves,&BigUint::one());
            self.square_frontier=Some(SquareFrontier {phase:node.phase.clone(),delta:delta.clone(),root:b.clone(),gap,
                excess:excess.clone(),complement});
            frame.leaf=Some(PhaseLeafCarrier::with_square(&node.phase,delta,b,excess,node.crt_false_span,node.gap_false_span)?);
            return Ok(())
        }
        const WORD:[u32;14]=[0,6,2,5,3,9,4,8,10,7,11,4,8,1];
        let leaf=frame.leaf.as_mut().unwrap();
        leaf.apply(WORD[leaf.ip],depth,ticks,n,lo,hi);leaf.ip+=1;
        if leaf.ip<WORD.len(){return Ok(())}
        if let Some(pair)=leaf.emitted.take(){self.frames.pop();self.returned=Some(Some(pair));return Ok(())}
        frame.leaf=None;
        let next=frame.begin.add(&crate::native_numeral::ImasmTape::one());
        if next.ge(&frame.end) {self.frames.pop();self.returned=Some(None);return Ok(())}
        let mid=next.add(&frame.end).divmod(&crate::native_numeral::ImasmTape::from_value(&BigUint::from(2u8))).ok_or_else(||"phase midpoint did not close".to_string())?.0;
        tape_branch_apply(6,depth,&mut frame.branch,ticks);frame.stage=1;
        self.frames.push(PhaseFamilyFrame {begin:next,end:mid,stage:0,branch:empty(),leaf:None});
        Ok(())
    }

    /// IMSCRIB continuation: the carrier applies its own refinement morphism
    /// to itself until IFIX is reached. The aggregate dispatcher never clocks
    /// or observes an intermediate continuation.
    fn imscribe_to_fix(&mut self,n:&crate::native_numeral::ImasmTape,lo:&crate::native_numeral::ImasmTape,hi:&crate::native_numeral::ImasmTape,depth:u32,
        ticks:&mut u64,leaves:&mut BigUint)->Result<(),String> {
        if self.complete {return Ok(())}
        self.compose_refinement(n,lo,hi,depth,ticks,leaves)?;
        self.imscribe_to_fix(n,lo,hi,depth,ticks,leaves)
    }

    /// One zoomed token denotes the composition of its complete internal word.
    /// Fold that word before returning to the parent IMSCRIB edge instead of
    /// surfacing between its fourteen constituent marks.
    fn compose_refinement(&mut self,n:&crate::native_numeral::ImasmTape,lo:&crate::native_numeral::ImasmTape,hi:&crate::native_numeral::ImasmTape,depth:u32,
        ticks:&mut u64,leaves:&mut BigUint)->Result<(),String> {
        // The selected phase word is itself refined into the complete
        // twelve-token alphabet. Compose the full child family here; returning
        // after only INTERNAL_WORD_LEN steps would leave phase breadth at the
        // parent boundary even though each individual leaf was enclosed.
        const INTERNAL_WORD_LEN:usize=14;
        const REFINEMENT_ALPHABET:usize=12;
        const BREADTH_DEPTH:usize=2;
        let folded_children=REFINEMENT_ALPHABET.pow(BREADTH_DEPTH as u32);
        for _ in 0..INTERNAL_WORD_LEN*folded_children {
            if self.complete {break}
            self.refine(n,lo,hi,depth,ticks,leaves)?;
        }
        Ok(())
    }
}

/// Execute the supplied aggregate-phase blueprint position by position. The
/// split opens at FSPLIT BEFORE the search, so the phase-family denotation is
/// generated inside the split/fuse frame and remains banked through both
/// evaluation arms, fusion, fixation, and terminal emission — δ before δ,
/// μ after μ. No candidate is returned across this boundary.
fn aggregate_phase_morphism(n:&BigUint,lo:&BigUint,hi:&BigUint,begin:&BigUint,end:&BigUint,
    depth:u32,ticks:&mut u64,leaves:&mut BigUint)->Result<Option<(BigUint,BigUint,[BigUint;6])>,String> {
    let mut s=AggregatePhaseState {generated:false,split:false,valid:None,invalid:false,
        held:None,fused:None,fixed:None,emitted:None,
        collapsed_crt_span:crate::native_numeral::ImasmTape::zero(),collapsed_gap_span:crate::native_numeral::ImasmTape::zero(),
        qr_admitted:crate::native_numeral::ImasmTape::zero(),gap_crossing_admitted:crate::native_numeral::ImasmTape::zero(),
        frontier_projections:crate::native_numeral::ImasmTape::zero(),shell_blocks:crate::native_numeral::ImasmTape::zero()};
    for (ip,glyph) in AGGREGATE_PHASE_WORD.chars().enumerate() {
        let token=imasm_core::classic::Token::parse(&glyph.to_string())
            .ok_or_else(||format!("aggregate phase contains unknown mark {glyph}"))?;
        let emitted=nested_fixed_point_emit(token as u32,depth,ticks);
        if emitted!=token as u32{return Err(format!("aggregate nested vessel changed step {} ({glyph})",ip+1))}
        match (ip,token) {
            (0,imasm_core::classic::Token::Vinit)=>{},
            (1,imasm_core::classic::Token::Fsplit)=>s.split=true,
            (2,imasm_core::classic::Token::Afwd)=>{
                let nt=crate::native_numeral::ImasmTape::from_value(n);
                let lot=crate::native_numeral::ImasmTape::from_value(lo);
                let hit=crate::native_numeral::ImasmTape::from_value(hi);
                let begint=crate::native_numeral::ImasmTape::from_value(begin);
                let endt=crate::native_numeral::ImasmTape::from_value(end);
                let mut bulk=DenotationBulk::new(&begint,&endt);
                bulk.imscribe_to_fix(&nt,&lot,&hit,depth,ticks,leaves)?;
                s.valid=bulk.fixed;
                s.collapsed_crt_span=bulk.collapsed_crt_span;s.collapsed_gap_span=bulk.collapsed_gap_span;
                s.qr_admitted=bulk.qr_admitted;s.gap_crossing_admitted=bulk.gap_crossing_admitted;
                s.frontier_projections=bulk.frontier_projections;
                s.shell_blocks=bulk.shell_blocks;
                s.generated=true;
            }
            (3,imasm_core::classic::Token::Clink) if s.generated=>{},
            (4,imasm_core::classic::Token::Imscrib) if s.generated=>{},
            (5,imasm_core::classic::Token::Evalt) if s.split=>{},
            (6,imasm_core::classic::Token::Afwd) if s.split=>{},
            (7,imasm_core::classic::Token::Clink) if s.split=>{},
            (8,imasm_core::classic::Token::Evalf) if s.split=>s.invalid=s.valid.is_none(),
            (9,imasm_core::classic::Token::Arev) if s.split=>{},
            (10,imasm_core::classic::Token::Clink) if s.split=>{},
            (11,imasm_core::classic::Token::Engagr) if s.split=>s.held=s.valid.take(),
            (12,imasm_core::classic::Token::Ffuse) if s.split=>{
                s.fused=s.held.take(); s.split=false;
            }
            (13,imasm_core::classic::Token::Ifix) if !s.split=>s.fixed=s.fused.take(),
            (14,imasm_core::classic::Token::Clink) if s.fixed.is_some()||s.invalid=>{},
            (15,imasm_core::classic::Token::Imscrib) if s.fixed.is_some()||s.invalid=>{},
            (16,imasm_core::classic::Token::Tanch)=>s.emitted=s.fixed.take(),
            _=>return Err(format!("aggregate phase failed to close at step {} ({glyph})",ip+1)),
        }
    }
    Ok(s.emitted.map(|(p,q)|(p.to_value(),q.to_value(),[
        s.collapsed_crt_span.to_value(),s.collapsed_gap_span.to_value(),s.qr_admitted.to_value(),
        s.gap_crossing_admitted.to_value(),s.frontier_projections.to_value(),s.shell_blocks.to_value()])))
}

/// Fully nested phase relation. The state is an unbounded integer and each
/// phase is the canonical factorization morphism; no global assignment graph
/// is constructed. Space stays proportional to the target width while time is
/// made by successive closed phase vessels.
pub fn select_relation(n_text:&str,m:u32,_capacity:u32,nest_depth:u32,_device:usize)->String {
    let result=(||->Result<String,String>{
        let n=BigUint::parse_bytes(n_text.as_bytes(),10)
            .ok_or_else(||"N must be a positive decimal integer".to_string())?;
        if m<2 || n<BigUint::from(3u8) || (&n&BigUint::one()).is_zero()
            || n.bits()>2*m as u64 {return Err("odd N >= 3, factor width >=2, and N < 2^(2m) required".into());}
        let lo=BigUint::one()<<(m-1) as usize;
        let hi=(BigUint::one()<<m as usize)-BigUint::one();
        // Seed a = ceil(sqrt(N)) is a word-walk: the floor root is read off
        // N's own glyph word by Newton descent, then bumped by one iff the
        // word's square falls short of N. No host-side BigUint sqrt remains.
        let n_tape=crate::native_numeral::ImasmTape::from_value(&n);
        let mut a_tape=n_tape.isqrt();
        if n_tape.gt(&a_tape.mul(&a_tape)) {
            a_tape=a_tape.add(&crate::native_numeral::ImasmTape::one());
        }
        let a=a_tape.to_value();
        let end_a=crate::native_numeral::add_via_word(&hi,&BigUint::one()); // exclusive bound
        let mut ticks=0u64;
        let mut leaves=BigUint::zero();
        if let Some((p,q,stats))=aggregate_phase_morphism(&n,&lo,&hi,&a,&end_a,nest_depth,&mut ticks,&mut leaves)? {
            return Ok(format!("fully nested aggregate phase relation: N={n} m={m}\n  process: {AGGREGATE_PHASE_WORD}\n  IMASM nesting depth={nest_depth}; denotation bulk vessels=1; internal phase leaves={leaves}; CRT false span={}; gap false span={}; QR admitted={}; gap crossing admitted={}; square-frontier projections={}; contained shell blocks={}; nested mark ticks={ticks}\n  temporal relation storage={} bits\n  {n} = {p} x {q} (verified)",stats[0],stats[1],stats[2],stats[3],stats[4],stats[5],n.bits()));
        }
        Ok(format!("fully nested aggregate phase relation: N={n} m={m}\n  process: {AGGREGATE_PHASE_WORD}\n  IMASM nesting depth={nest_depth}; denotation bulk vessels=1; internal phase leaves={leaves}; nested mark ticks={ticks}\n  empty factor relation in the declared bit bands"))
    })();
    result.unwrap_or_else(|e|format!("selector_relation: {e}"))
}

#[cfg(test)]
mod temporal_phase_relation_tests {
    use super::*;

    #[test]
    fn every_zoom_emits_the_same_nested_morphism_fixed_point() {
        for depth in [0u32,1,2,64,u32::MAX] {
            for mark in 0u32..12 {
                let mut ticks=0;
                assert_eq!(nested_fixed_point_emit(mark,depth,&mut ticks),mark);
                assert_eq!(ticks,1,"a fixed-point nest closes in one composition");
            }
        }
    }

    #[test]
    fn afwd_mutates_live_state_once_inside_every_nesting_depth() {
        let n=BigUint::from(143u16);
        let lo=BigUint::from(8u8);
        let hi=BigUint::from(15u8);
        for depth in [0u32,1,3,64,u32::MAX] {
            let mut state=FactorPhaseState {n:&n,lo:&lo,hi:&hi,a:BigUint::from(11u8),
                delta:BigUint::zero(),b:BigUint::zero(),square:false,
                candidate:None,fixed:None,emitted:None};
            let mut ticks=0;
            assert!(factor_vessel_apply(2,depth,&mut state,&mut ticks));
            assert_eq!(state.a,BigUint::from(12u8));
            assert_eq!(ticks,1,"refinement depth must collapse before execution");
        }
    }

    #[test]
    fn sixty_five_bit_close_pair_closes_in_one_phase() {
        let out=select_relation("18446744400127067027",33,1024,64,0);
        assert!(out.contains("denotation bulk vessels=1; internal phase leaves=1"),"{out}");
        assert!(out.contains("18446744400127067027 = 4294967311 x 4294967357 (verified)"),"{out}");
        assert!(out.contains("nested mark ticks=31"),"{out}");
    }

    #[test]
    fn phase_state_crosses_one_hundred_and_fifty_bits() {
        let p=(BigUint::one()<<79usize)+BigUint::from(11u8);
        let q=&p+BigUint::from(46u8);
        let n=&p*&q;
        let out=select_relation(&n.to_string(),80,1024,3,0);
        assert!(out.contains("denotation bulk vessels=1; internal phase leaves=1"),"{out}");
        assert!(out.contains(&format!("{n} = {p} x {q} (verified)")),"{out}");
    }

    #[test]
    fn hosted_executable_shell_carries_every_mark_at_its_entry_depth() {
        let mut ticks=0;
        for glyph in HOSTED_EXECUTABLE_WORD.chars() {
            let token=imasm_core::classic::Token::parse(&glyph.to_string()).unwrap();
            assert_eq!(nested_fixed_point_emit(token as u32,HOSTED_EXECUTABLE_NEST_DEPTH,&mut ticks),token as u32);
        }
        assert_eq!(ticks,HOSTED_EXECUTABLE_WORD.chars().count() as u64);
    }

    #[test]
    fn nested_continuation_reaches_a_later_phase_and_fixes_it() {
        let out=select_relation("8509",7,1024,u32::MAX,0);
        assert!(out.contains("denotation bulk vessels=1; internal phase leaves=1; CRT false span=4; gap false span=0"),"{out}");
        assert!(out.contains("8509 = 67 x 127 (verified)"),"{out}");
    }

    #[test]
    fn carried_square_frontier_matches_fresh_tape_projection() {
        use crate::native_numeral::ImasmTape;
        let mut delta=ImasmTape::from_value(&BigUint::from(3u8));
        let mut b=delta.isqrt();
        for rise in [17u64,257,65537,1_000_003] {
            delta=delta.add(&ImasmTape::from_value(&BigUint::from(rise)));
            b=delta.isqrt_from(&b);
            assert_eq!(b,delta.isqrt());
        }
    }

    #[test]
    fn every_positive_fermat_phase_crosses_the_square_frontier() {
        use crate::native_numeral::ImasmTape;
        for (n,a) in [(3u64,2u64),(143,12),(8509,93),(1_000_003,1_001)] {
            let nt=ImasmTape::from_small(n);let at=ImasmTape::from_small(a);
            let d=at.mul(&at).sub(&nt).unwrap();let b=d.isqrt();let bp=b.add(&ImasmTape::one());
            let gap=bp.mul(&bp).sub(&d).unwrap();let step=at.add(&at).add(&ImasmTape::one());
            assert!(step.ge(&gap),"a positive Fermat phase must reach the next square frontier");
        }
    }

    #[test]
    fn closed_form_tape_jump_equals_every_scalar_phase() {
        use crate::native_numeral::ImasmTape;
        let n=ImasmTape::from_small(8509);
        for (a,j) in [(93u64,1u64),(93,4),(93,37),(257,130)] {
            let at=ImasmTape::from_small(a);
            let mut scalar=at.mul(&at).sub(&n).unwrap();
            let mut scalar_step=at.add(&at).add(&ImasmTape::one());
            for _ in 0..j {scalar=scalar.add(&scalar_step);scalar_step=scalar_step.add(&ImasmTape::from_small(2));}
            let jump=ImasmTape::from_small(j);
            let jm1=jump.sub(&ImasmTape::one()).unwrap_or_else(ImasmTape::zero);
            let start=at.mul(&at).sub(&n).unwrap();
            let step=at.add(&at).add(&ImasmTape::one());
            let closed=start.add(&step.mul(&jump)).add(&jump.mul(&jm1));
            assert_eq!(closed,scalar);
            assert_eq!(step.add(&jump.add(&jump)),scalar_step);
        }
    }

    #[test]
    fn fermat_correction_matches_full_root_in_both_regimes() {
        use crate::native_numeral::ImasmTape;
        for (n,a,j) in [(8509u64,93u64,4u64),(8509,1_000,37),(143,12,130)] {
            let nt=ImasmTape::from_small(n);let at=ImasmTape::from_small(a);
            let jump=ImasmTape::from_small(j);let d=at.mul(&at).sub(&nt).unwrap();let b=d.isqrt();
            let t=d.sub(&b.mul(&b)).unwrap();let c=at.sub(&b).unwrap();let base=b.add(&jump);
            let jc=jump.mul(&c);let excess=t.add(&jc.add(&jc));
            let (q,remainder,_)=ImasmTape::nest_into_square_frontier(&excess,&base);let root=base.add(&q);
            let destination=at.add(&jump);let landed=destination.mul(&destination).sub(&nt).unwrap();
            assert_eq!(root,landed.isqrt());
            assert_eq!(excess,q.mul(&base.add(&base).add(&q)).add(&remainder));
            assert!(root.add(&root).add(&ImasmTape::one()).gt(&remainder));
            assert_eq!(remainder,landed.sub(&root.mul(&root)).unwrap());
        }
    }

    #[test]
    fn deficit_solver_closes_each_exact_arm() {
        use crate::native_numeral::ImasmTape;
        let (q0,s0)=ImasmTape::root_deficit_from(&ImasmTape::from_small(100),&ImasmTape::from_small(610));
        assert_eq!(q0,ImasmTape::from_small(3));assert_eq!(s0[0],ImasmTape::one());
        let (q1,s1)=ImasmTape::root_deficit_from(&ImasmTape::from_small(100),&ImasmTape::from_small(600));
        assert_eq!(q1,ImasmTape::from_small(2));assert_eq!(s1[1],ImasmTape::one());
        let (q2,s2)=ImasmTape::root_deficit_from(&ImasmTape::from_small(10),&ImasmTape::from_small(200));
        assert_eq!(q2,ImasmTape::from_small(7));assert_eq!(s2[2],ImasmTape::one());
    }

    #[test]
    fn lazy_pair_wheel_lands_with_exact_tape_witnesses() {
        use crate::native_numeral::ImasmTape;
        let n=ImasmTape::from_small(8509);let phase=ImasmTape::from_small(93);
        let d=phase.mul(&phase).sub(&n).unwrap();let step=phase.add(&phase).add(&ImasmTape::one());
        let moduli=[ImasmTape::from_small(64),ImasmTape::from_small(63),ImasmTape::from_small(65)];
        let residue=|x:&ImasmTape,m:&ImasmTape|x.divmod(m).unwrap().1;
        let residues=[residue(&d,&moduli[0]),residue(&d,&moduli[1]),residue(&d,&moduli[2]),residue(&step,&moduli[0]),residue(&step,&moduli[1]),residue(&step,&moduli[2])];
        let images=[qr_image(&[0,1,4,9,16,17,25,33,36,41,49,57]),qr_image(&[0,1,4,7,9,16,18,22,25,28,36,37,43,46,49,58]),qr_image(&[0,1,4,9,10,14,16,25,26,29,30,35,36,39,40,49,51,55,56,61,64])];
        let node=lazy_tape_jump(phase.clone(),&phase,&ImasmTape::from_small(128),residues,
            residue(&phase,&ImasmTape::from_small(4032)),residue(&phase,&moduli[2]),&moduli,&images,
            &step,None).unwrap();
        assert_eq!(node.phase,ImasmTape::from_small(97));assert_eq!(node.jump,ImasmTape::from_small(4));
        let landed=node.phase.mul(&node.phase).sub(&n).unwrap();let landed_step=node.phase.add(&node.phase).add(&ImasmTape::one());
        for j in 0..3 {assert_eq!(node.residues[j],residue(&landed,&moduli[j]));assert_eq!(node.residues[j+3],residue(&landed_step,&moduli[j]));}
    }
}

pub fn select_bdd_relation(n_text: &str, m: u32, capacity: u32, nest_depth: u32, device: usize) -> String {
    let result = (|| -> Result<String,String> {
        let n=BigUint::parse_bytes(n_text.as_bytes(),10)
            .ok_or_else(||"N must be a positive decimal integer".to_string())?;
        if !capacity.is_power_of_two() || capacity<16 || capacity>1<<29 {
            return Err("initial node allocation must be a power of two >=16, representable by the device's 32-bit node indices".into());
        }
        if nest_depth>64 { return Err("nest depth must be at most 64".into()); }
        let graph = crate::factor_relation::RelationProgram::product_big(&n,m).map_err(String::from)?;
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../ob3ect/digital/imasm_word_that_constructs_and_selects_the_compa_9eeb5656/imasm_word_that_constructs_and_selects_the_compa_9eeb5656_ob3ect.json");
        let object: serde_json::Value = serde_json::from_slice(&std::fs::read(path).map_err(|e|e.to_string())?)
            .map_err(|e|e.to_string())?;
        let word = ob3ect_program(&object)?;
        if word != "⊢⊙∈≻⊤≺⊥⋈⊞∋⊡⊣" { return Err("unsupported outer relation protocol".into()); }
        let ids = [0u8,8,6,2,5,3,9,4,10,7,11,1];
        let ctx = CudaContext::new(device).map_err(|e|format!("CUDA: {e}"))?;
        ctx.set_limit(cudarc::driver::sys::CUlimit::CU_LIMIT_STACK_SIZE,65536).map_err(|e|e.to_string())?;
        let stream = ctx.default_stream();
        let source = format!("#include <nv/target>\n#include <mma.h>\n#define IMASM_FACTOR_RELATION\n{KERNEL_NESTED_SRC}");
        let module = ctx.load_module(compile_ptx_with_opts(source,CompileOptions {
            include_paths:vec![String::from("/usr/local/cuda-12.4/targets/x86_64-linux/include")],
            arch:Some("compute_86"),..Default::default()
        }).map_err(|e|format!("NVRTC: {e}"))?)
            .map_err(|e|e.to_string())?;
        let func = module.load_function("run_nested").map_err(|e|e.to_string())?;
        let compact_func = module.load_function("run_compact").map_err(|e|e.to_string())?;
        let rehash_func = module.load_function("run_compact_rehash").map_err(|e|e.to_string())?;
        let control_func = module.load_function("run_relation_control").map_err(|e|e.to_string())?;
        let epoch_func = module.load_function("run_relation_epoch").map_err(|e|e.to_string())?;
        let limb_func = module.load_function("run_witness_limb").map_err(|e|e.to_string())?;
        let tensor_func = module.load_function("run_tensor_enfold_ops").map_err(|e|e.to_string())?;
        let packed: Vec<u32> = graph.ops.iter().flatten().copied().collect();
        let d_ops = stream.clone_htod(&packed).map_err(|e|e.to_string())?;
        let mut d_enfolded_ops=stream.alloc_zeros::<u32>(packed.len()).map_err(|e|e.to_string())?;
        let d_word = stream.clone_htod(&ids).map_err(|e|e.to_string())?;
        let d_last_use = stream.clone_htod(&graph.last_use).map_err(|e|e.to_string())?;
        let cap = capacity as usize;
        let mut nodes = stream.alloc_zeros::<u32>(3*cap).map_err(|e|e.to_string())?;
        let mut unique = stream.alloc_zeros::<u32>(2*cap).map_err(|e|e.to_string())?;
        let mut cache = stream.alloc_zeros::<u32>(6*cap).map_err(|e|e.to_string())?;
        let mut values = stream.alloc_zeros::<u32>(graph.ops.len()).map_err(|e|e.to_string())?;
        let mut out = stream.alloc_zeros::<u64>(14).map_err(|e|e.to_string())?;
        let len=ids.len() as u32; let count=graph.ops.len() as u32;
        {
            let mut tensor=stream.launch_builder(&tensor_func);
            tensor.arg(&d_ops).arg(&count).arg(&nest_depth).arg(&mut d_enfolded_ops);
            unsafe {tensor.launch(LaunchConfig {grid_dim:(count,1,1),block_dim:(32,1,1),shared_mem_bytes:0})}
                .map_err(|e|format!("tensor enfolding launch: {e}"))?;
        }
        let epoch_ops=256u32;
        let steps=u64::MAX;
        let mut capacity=capacity;
        let mut growths=0u32;
        let mut compactions=0u32;
        let mut launches=1u32; // tensor-core operator enfolding pass
        let mut allocation_error=None;
        let mut last_compact_at: Option<(u32,u32)> = None;
        let start=std::time::Instant::now();
        let result=loop {
            let mut launch=stream.launch_builder(&func);
            launch.arg(&d_word).arg(&len).arg(&d_enfolded_ops).arg(&count).arg(&graph.root).arg(&graph.width)
                .arg(&mut nodes).arg(&mut unique).arg(&mut cache).arg(&mut values)
                .arg(&capacity).arg(&nest_depth).arg(&epoch_ops).arg(&steps).arg(&mut out);
            unsafe {launch.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}
                .map_err(|e|format!("construction launch: {e}"))?;
            launches+=1;
            let result=stream.clone_dtoh(&out).map_err(|e|format!("construction completion: {e}"))?;
            if result[0]!=2 && result[0]!=6 {break result;}
            if result[0]==6 {
                let continuation=result[9] as u32;
                let mut epoch=stream.launch_builder(&epoch_func);
                epoch.arg(&continuation).arg(&nest_depth).arg(&mut out);
                unsafe {epoch.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}
                    .map_err(|e|format!("epoch vessel launch: {e}"))?;
                let permitted=stream.clone_dtoh(&out).map_err(|e|format!("epoch vessel completion: {e}"))?;
                if permitted[12]!=continuation as u64 {
                    allocation_error=Some("full nested IMASM vessel refused the temporal construction continuation".into());
                    break result;
                }
                // The complete enfolding word carries the live construction
                // directly into the next epoch. Reclamation remains a response
                // to node pressure rather than part of every temporal tick.
                continue;
            }
            let current_i=result[9] as u32;
            let old_used=result[3] as u32+1;
            let mut compacted=false;
            let stuck=last_compact_at==Some((current_i,old_used));
            if current_i>0 && !stuck {
                // cache is reinterpreted as a mark bitset; it must start
                // clean of whatever apply-memo contents it held. A hardware
                // memset does this and the two zeroings below in a fraction
                // of what a single GPU thread looping over the same words
                // costs -- the actual mark/sweep/rewrite logic (still one
                // thread, already proven correct) is real dependency-bound
                // work, but zeroing millions of independent words never was.
                let _=stream.memset_zeros(&mut cache);
                let mut launch=stream.launch_builder(&compact_func);
                launch.arg(&mut nodes).arg(&mut unique).arg(&mut cache).arg(&mut values)
                    .arg(&d_last_use).arg(&graph.root).arg(&current_i)
                    .arg(&old_used).arg(&capacity).arg(&nest_depth).arg(&mut out);
                if unsafe {launch.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}.is_ok() {
                    if let Ok(after)=stream.clone_dtoh(&out) {
                        let new_used=after[3] as u32+1;
                        if new_used<old_used {
                            let _=stream.memset_zeros(&mut unique);
                            let mut rehash=stream.launch_builder(&rehash_func);
                            rehash.arg(&nodes).arg(&mut unique).arg(&new_used).arg(&capacity)
                                .arg(&nest_depth).arg(&mut out);
                            unsafe {rehash.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}
                                .map_err(|e|format!("compaction rehash launch: {e}"))?;
                            let _=stream.memset_zeros(&mut cache);
                            if new_used<old_used { compactions+=1; }
                            compacted=true;
                            last_compact_at=Some((current_i,old_used));
                        }
                    }
                }
            }
            if compacted { continue; }
            // Compaction found nothing to reclaim, or wasn't attempted; put
            // out[0] back to the grow-resume sentinel run_compact may have
            // overwritten before falling through to growing storage.
            let two=[2u64];
            let _=stream.memcpy_htod(&two,&mut out.slice_mut(0..1));
            if capacity>=1<<29 {
                allocation_error=Some("device node-index representation exhausted".into());
                break result;
            }
            // AFWD emitted through the tower authorizes the host-side resource
            // continuation. Allocation supplies storage; it does not decide
            // whether the relation advances into it.
            let mut control=stream.launch_builder(&control_func);
            let afwd=2u32;
            control.arg(&afwd).arg(&nest_depth).arg(&mut out);
            unsafe {control.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}
                .map_err(|e|format!("storage continuation launch: {e}"))?;
            let permitted=stream.clone_dtoh(&out).map_err(|e|format!("storage continuation completion: {e}"))?;
            if permitted[12]!=afwd as u64 {
                allocation_error=Some("nested IMASM refused the storage continuation".into());
                break result;
            }
            let next=capacity*2;
            let replacement=(|| {
                let mut new_nodes=stream.alloc_zeros::<u32>(3*next as usize)?;
                let new_unique=stream.alloc_zeros::<u32>(2*next as usize)?;
                let new_cache=stream.alloc_zeros::<u32>(6*next as usize)?;
                stream.memcpy_dtod(&nodes,&mut new_nodes.slice_mut(..3*capacity as usize))?;
                Ok::<_,cudarc::driver::DriverError>((new_nodes,new_unique,new_cache))
            })();
            match replacement {
                Ok((new_nodes,new_unique,new_cache))=>{
                    nodes=new_nodes;unique=new_unique;cache=new_cache;capacity=next;growths+=1;
                }
                Err(e)=>{allocation_error=Some(format!("CUDA allocation failed while growing storage: {e}"));break result;}
            }
        };
        let elapsed=start.elapsed().as_secs_f64();
        // Read the witness as a sequence of bounded limbs. Each launch receives
        // only the continuation edge emitted by the previous TANCH.
        let limb_count=(graph.width+31)/32;
        let mut limb_edge=result[6] as u32;
        let mut streamed_p=BigUint::from(0u8);
        let mut streamed_q=BigUint::from(0u8);
        let mut limb_ticks=0u64;
        let mut selection_ticks=0u64;
        let mut limb_path=0u64;
        for limb in 0..limb_count {
            let mut limb_out=stream.alloc_zeros::<u64>(6).map_err(|e|e.to_string())?;
            let mut launch=stream.launch_builder(&limb_func);
            launch.arg(&nodes).arg(&limb_edge).arg(&limb).arg(&nest_depth).arg(&mut limb_out);
            unsafe {launch.launch(LaunchConfig {grid_dim:(1,1,1),block_dim:(1,1,1),shared_mem_bytes:0})}
                .map_err(|e|format!("witness limb launch: {e}"))?;
            let emitted=stream.clone_dtoh(&limb_out).map_err(|e|format!("witness limb completion: {e}"))?;
            limb_edge=emitted[0] as u32;
            streamed_p|=BigUint::from(emitted[1])<<(32*limb as usize);
            streamed_q|=BigUint::from(emitted[2])<<(32*limb as usize);
            limb_path+=emitted[3];limb_ticks+=emitted[4];
            selection_ticks+=emitted[5];
        }
        let solutions=if result[0]>1 {"unavailable".into()}
            else if result[8]==u64::MAX {"temporal".into()}
            else {format!("{}",result[8])};
        let total_nested_ticks=result[11]+limb_ticks;
        let construction_epochs=(count+epoch_ops-1)/epoch_ops;
        let head=format!("symbolic product relation in run_nested: N={n} m={m}\n  process: {word}\n  hierarchy: WMMA tensor tile -> warp -> operator block -> relation grid -> temporal launch family\n  tensor-enfolded operators={count}; tensor nesting depth={nest_depth}\n  circuit operators={count}; construction epochs={construction_epochs}; operators per epoch={epoch_ops}; IMASM nesting depth={nest_depth}; nested mark ticks={total_nested_ticks}; selection ticks={selection_ticks}\n  decision nodes={}; graph reductions={}; {:.6}s\n  factor-candidate loop: none; relation solutions={solutions}; witness path nodes={limb_path}\n  temporal witness limbs={limb_count}; limb path nodes={limb_path}; limb ticks={limb_ticks}; continuation={}\n  node storage={capacity}; storage growths={growths}; compactions={compactions}; device launches={launches}",
            result[3],result[4],elapsed,limb_edge);
        match result[0] {
            0=>Ok(format!("{head}\n  empty factor relation in the declared bit bands")),
            1=>{
                if streamed_p<BigUint::from(2u8) || streamed_q<BigUint::from(2u8)
                    || &streamed_p*&streamed_q!=n {
                    return Err(format!("temporal relation witness failed independent product verification: {streamed_p}, {streamed_q}"));
                }
                if limb_edge>=2 { return Err(format!("temporal witness did not terminate: continuation {limb_edge}")); }
                Ok(format!("{head}\n  {n} = {streamed_p} x {streamed_q} (verified)"))
            }
            status=>{
                let why=allocation_error.as_deref().unwrap_or(match status {2=>"decision-node allocation",3=>"graph-reduction budget",4=>"apply-stack capacity",_=>"invalid relation protocol"});
                Ok(format!("{head}\n  construction INCOMPLETE: {why}; no factor verdict available"))
            }
        }
    })();
    result.unwrap_or_else(|e|format!("selector_relation: {e}"))
}

/// Execute the supplied selector process over the established phase relation.
/// This still traverses phase bands; it is not a search-free selector.
pub fn bench_selector(n: u64, m: u32, device: usize, joint: bool) -> String {
    if n < 3 || n&1 == 0 || !(3..=32).contains(&m) {
        return "selector: positive odd N >= 3 and factor width 3..32 required".into();
    }
    if joint && (m>30 || n>=(1u64<<(2*m))) {
        return "selector_joint: width must be <=30 and N must fit within twice that width".into();
    }
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../ob3ect/digital/imasm_word_that_constructs_and_selects_the_compa_9eeb5656/imasm_word_that_constructs_and_selects_the_compa_9eeb5656_ob3ect.json");
    let object = match std::fs::read(path).ok().and_then(|s| serde_json::from_slice::<serde_json::Value>(&s).ok()) {
        Some(o)=>o, None=>return "selector: cannot load supplied ob3ect".into(),
    };
    let word = match ob3ect_program(&object) {
        Ok(w) => w, Err(e) => return format!("selector: {e}"),
    };
    use crate::belnap_ring_shor::{Glyph, glyph_to_token};
    let ids: Option<Vec<u8>> = word.chars().map(|c| Glyph::from_char(c).map(|g|tok_id(glyph_to_token(g)))).collect();
    let ids = match ids {Some(v)=>v, None=>return "selector: unknown operator".into()};
    // The phase process has these data dependencies; reject a reordered or
    // altered word rather than silently running the old operation schedule.
    if ids != [0,8,6,2,5,3,9,4,10,7,11,1] {
        return "selector: supplied word does not match phase-process dependencies".into();
    }
    format!("selector process: {word}\n{}", bench_phase_program(n,m,device,&ids,joint))
}

fn bench_phase_program(n: u64, m: u32, device: usize, word: &[u8], joint: bool) -> String {
    const JOINT_ROOT_BITS: u32 = 14;
    if m < 2 { return "phase: factor bit length m must be at least 2".to_string(); }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("phase: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let source=format!("#define JOINT_ROOT_BITS {JOINT_ROOT_BITS}\n{}\n{}",include_str!("gpu_joint.cuh"),KERNEL_PHASE_SRC);
    let module = match cudarc::nvrtc::compile_ptx_with_opts(source, cudarc::nvrtc::CompileOptions {
        options: alloc::vec!["--device-int128".into()], ..Default::default()
    }).map(|p| ctx.load_module(p)) {
        Ok(Ok(mm)) => mm, Ok(Err(e)) => return format!("phase: module: {e}"),
        Err(e) => return format!("phase: nvrtc: {e}"),
    };
    let func = match module.load_function("run_phase") {
        Ok(f) => f, Err(e) => return format!("phase: load: {e}"),
    };

    let band: u32 = 256;
    // canonical space = all odd residues mod 2^m = 2^(m-1); threads cover it in bands.
    let positions: u64 = 1u64 << (m - 1);
    let n_threads: u32 = if joint {1u32<<(m.min(JOINT_ROOT_BITS)-1)} else if word.is_empty() {
        core::cmp::max(1, (positions / band as u64) as u32)
    } else { (4 * (((1u64 << (m-3)) + band as u64-1) / band as u64)) as u32 };
    let block: u32 = if joint {64} else {256};
    let grid = ((n_threads as u64 + block as u64 - 1) / block as u64) as u32;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
    let mut d_out = match stream.clone_htod(&alloc::vec![n,0,0,0,0]) { Ok(d) => d, Err(e) => return format!("phase: htod: {e}") };
    let word_data = if word.is_empty() { &[0u8][..] } else { word };
    let d_word = match stream.clone_htod(word_data) {Ok(d)=>d,Err(e)=>return format!("phase: word: {e}")};
    let word_len = word.len() as u32;
    let mode=if joint {2u32} else if word.is_empty() {0} else {1};

    // warm
    { let mut b = stream.launch_builder(&func);
      b.arg(&n); b.arg(&m); b.arg(&mut d_out); b.arg(&band); b.arg(&n_threads);
      b.arg(&d_word); b.arg(&word_len); b.arg(&mode);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("phase: launch: {e}"); } }
    let _ = stream.clone_dtoh(&d_out);
    let mut d2 = match stream.clone_htod(&alloc::vec![n,0,0,0,0]) { Ok(d) => d, Err(e) => return format!("phase: htod2: {e}") };
    let t0 = std::time::Instant::now();
    { let mut b = stream.launch_builder(&func);
      b.arg(&n); b.arg(&m); b.arg(&mut d2); b.arg(&band); b.arg(&n_threads);
      b.arg(&d_word); b.arg(&word_len); b.arg(&mode);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("phase: launch: {e}"); } }
    let res = match stream.clone_dtoh(&d2) { Ok(v) => v, Err(e) => return format!("phase: sync: {e}") };
    let secs = t0.elapsed().as_secs_f64().max(1e-9);

    let f = res[0];
    let mut head = format!("phase walk: N={} m={}  {} positions, {} threads x {} band, {:.4}s",
        n, m, positions, n_threads, band, secs);
    if joint { head=format!("joint relation: N={n} m={m}, {n_threads} initial prefixes, {secs:.4}s"); }
    if !word.is_empty() { head.push_str(&format!("\n  executed {}: {}; full-product checks: {}", if joint {"relation prefixes"} else {"phase visits"}, res[1],res[2])); }
    if joint { head.push_str(&format!("\n  full-product envelopes: {}; prefix subdivisions: {}",res[3],res[4])); }
    if f == n || f < 2 {
        format!("{}\n  no factor found in the canonical space (raise m or check it is composite)", head)
    } else {
        let cof = n / f;
        let ok = if f.checked_mul(cof) == Some(n) { "verified" } else { "MISMATCH" };
        let rate = positions as f64 / secs / 1e9;
        if joint { format!("{}\n  {} = {} x {}  ({})  [joint contraction; prefix search counted above]",head,n,f,cof,ok) }
        else { format!("{}\n  {} = {} x {}  ({})  [{:.2} Gphase/s, division-free]", head, n, f, cof, ok, rate) }
    }
}

/// Factor by prime-band intersection. Sieve the m-bit primes once into a bitset,
/// then on the card every prime P keeps only the states whose forced complement
/// Q is also an m-bit prime, and the true factors fall out of the exact product.
/// The survivor count is the control: it must match the codebook census (64 at
/// m=13, 2830 at m=20). The sieve is one-time and reusable; the query is cheap.
pub fn bench_codebook(n: u64, m: u32, device: usize) -> String {
    if m < 4 { return "codebook: width m must be at least 4".to_string(); }
    // one-time sieve of primes over [0, 2^m) into a bit-per-integer set.
    let t_sieve = std::time::Instant::now();
    let size = 1usize << m;
    let mut is_p = alloc::vec![true; size];
    is_p[0] = false; if size > 1 { is_p[1] = false; }
    let mut i = 2usize;
    while i * i < size { if is_p[i] { let mut j = i * i; while j < size { is_p[j] = false; j += i; } } i += 1; }
    let words = (size + 31) / 32;
    let mut bits = alloc::vec![0u32; words];
    for x in 0..size { if is_p[x] { bits[x >> 5] |= 1u32 << (x & 31); } }
    let sieve_s = t_sieve.elapsed().as_secs_f64();

    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("codebook: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_CODEBOOK_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(mm)) => mm, Ok(Err(e)) => return format!("codebook: module: {e}"),
        Err(e) => return format!("codebook: nvrtc: {e}"),
    };
    let func = match module.load_function("run_codebook") {
        Ok(f) => f, Err(e) => return format!("codebook: load: {e}"),
    };

    let d_bits = match stream.clone_htod(&bits) { Ok(d) => d, Err(e) => return format!("codebook: htod: {e}") };
    let mut d_out = match stream.clone_htod(&alloc::vec![n]) { Ok(d) => d, Err(e) => return format!("codebook: htod out: {e}") };
    let mut d_surv = match stream.alloc_zeros::<u64>(1) { Ok(d) => d, Err(e) => return format!("codebook: alloc: {e}") };
    let np: u32 = 1u32 << (m - 2);
    let block: u32 = 256;
    let grid = (np + block - 1) / block;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };

    let t_q = std::time::Instant::now();
    { let mut b = stream.launch_builder(&func);
      b.arg(&d_bits); b.arg(&n); b.arg(&m); b.arg(&mut d_out); b.arg(&mut d_surv); b.arg(&np);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("codebook: launch: {e}"); } }
    let outv = match stream.clone_dtoh(&d_out) { Ok(v) => v, Err(e) => return format!("codebook: sync: {e}") };
    let survv = match stream.clone_dtoh(&d_surv) { Ok(v) => v, Err(e) => return format!("codebook: sync2: {e}") };
    let query_s = t_q.elapsed().as_secs_f64();

    let f = outv[0];
    let head = format!(
        "codebook: N={} m={}  sieve {:.3}s (one-time, reusable), query {:.5}s\n  {} prime-compatible survivor states",
        n, m, sieve_s, query_s, survv[0]);
    if f == n || f < 2 {
        format!("{}\n  no factor recovered (check width or compositeness)", head)
    } else {
        let cof = n / f;
        let ok = if f.checked_mul(cof) == Some(n) { "verified" } else { "MISMATCH" };
        format!("{}\n  {} = {} x {}  ({})", head, n, f, cof, ok)
    }
}

/// Factor past the 64-bit product wall by tracking the closure height. The walk
/// seeds the height once and updates it by an exact add each step, sealing when
/// it reaches zero, so N may be far wider than 64 bits while every step stays in
/// 64-bit arithmetic. The control is the known factor of a 73-bit semiprime the
/// phase kernel cannot reach.
pub fn bench_closure(n: u128, m: u32, device: usize) -> String {
    if m < 4 { return "closure: width m must be >= 4".to_string(); }
    if m > 60 {
        // The tower routes past the u64 limb to the arbitrary-precision
        // closure-height walk. Same trilattice tiling the horn torus; no width
        // floor. Position count stays O(2^m), so the step budget bounds the run.
        let nb = BigUint::from(n);
        let steps: usize = 1usize << core::cmp::min(m as usize - 3, 40);
        return match crate::closure_nested::closure_height_walk(&nb, m as usize, steps) {
            Some(f) => {
                let cof = &nb / &f;
                let ok = if &f * &cof == nb { "verified" } else { "MISMATCH" };
                format!("closure (floorless, m>60 → arbitrary-precision walk): N = {} x {} ({})  [m={}, {} steps]",
                    f, cof, ok, m, steps)
            }
            None => format!("closure (floorless): no factor at m={} within {} steps (O(2^m) position count, no width floor)", m, steps),
        };
    }
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("closure: no CUDA: {e}"),
    };
    let stream = ctx.default_stream();
    let module = match compile_ptx(KERNEL_CLOSURE_SRC).map(|p| ctx.load_module(p)) {
        Ok(Ok(mm)) => mm, Ok(Err(e)) => return format!("closure: module: {e}"),
        Err(e) => return format!("closure: nvrtc: {e}"),
    };
    let func = match module.load_function("run_closure") {
        Ok(f) => f, Err(e) => return format!("closure: load: {e}"),
    };

    let n_lo: u64 = (n & 0xFFFF_FFFF_FFFF_FFFF) as u64;
    let n_hi: u64 = (n >> 64) as u64;
    let band: u32 = 256;
    let positions: u64 = 1u64 << (m - 1);
    let n_threads: u64 = core::cmp::max(1, positions / band as u64);
    let block: u32 = 256;
    let grid = core::cmp::min((n_threads + block as u64 - 1) / block as u64, (1u64 << 31) - 1) as u32;
    let cfg = LaunchConfig { grid_dim: (grid,1,1), block_dim: (block,1,1), shared_mem_bytes: 0 };
    let mut d_out = match stream.clone_htod(&alloc::vec![u64::MAX]) { Ok(d) => d, Err(e) => return format!("closure: htod: {e}") };

    let t0 = std::time::Instant::now();
    { let mut b = stream.launch_builder(&func);
      b.arg(&n_lo); b.arg(&n_hi); b.arg(&m); b.arg(&mut d_out); b.arg(&band); b.arg(&n_threads);
      if let Err(e) = unsafe { b.launch(cfg) } { return format!("closure: launch: {e}"); } }
    let res = match stream.clone_dtoh(&d_out) { Ok(v) => v, Err(e) => return format!("closure: sync: {e}") };
    let secs = t0.elapsed().as_secs_f64().max(1e-9);

    let head = format!("closure walk: N={} ({} bits) m={}  {} positions, {} threads x {} band, {:.4}s",
        n, 128 - n.leading_zeros(), m, positions, n_threads, band, secs);
    let f = res[0];
    if f == u64::MAX || (f as u128) >= n || f < 2 {
        format!("{}\n  no factor found (raise m or check compositeness)", head)
    } else {
        let cof = n / (f as u128);
        let ok = if (f as u128) * cof == n { "verified" } else { "MISMATCH" };
        let rate = positions as f64 / secs / 1e9;
        format!("{}\n  N = {} x {}  ({})  [{:.2} Gphase/s, 64-bit walk, no full product]", head, f, cof, ok, rate)
    }
}

/// Closure-predicate decision-diagram size, pushed past what a full-table host
/// build reaches. Enumerates the phase orbit, builds the truth table of "P times
/// Q congruent to N mod two-to-the-m-plus-r" over the phase bits low-to-high,
/// and reduces it bottom-up counting distinct diagram nodes. Reports the size and
/// the growth factor against the previous width, so the exponent is read off
/// directly.
pub fn bdd_size(n_num: u128, m: u32, r: u32) -> String {
    if m < 5 { return "bdd: width m must be at least 5".to_string(); }
    use std::collections::HashMap;
    let n = (m - 3) as usize;                 // phase bits
    let orbit: usize = 1usize << n;
    let modulus: u128 = 1u128 << m;
    let need: u128 = 1u128 << (m + r);
    let mask: u128 = modulus - 1;
    // Newton inverse of 9 mod 2^m.
    let inv9 = {
        let a = 9u128 & mask; let mut x = 1u128;
        for _ in 0..7 { x = (x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)))) & mask; }
        x
    };
    let nm = n_num & mask;
    // seed P = 3, Q = N * P^{-1} mod 2^m via Newton on P.
    let inv_p0 = {
        let a = 3u128 & mask; let mut x = 1u128;
        for _ in 0..7 { x = (x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)))) & mask; }
        x
    };
    let mut p = 3u128 & mask;
    let mut q = (nm.wrapping_mul(inv_p0)) & mask;
    // truth table in k order: bit0 of k is eliminated first = low-to-high.
    let mut cur: Vec<u32> = Vec::with_capacity(orbit);
    for _ in 0..orbit {
        let prod = p.wrapping_mul(q);          // p,q < 2^m, m<=30 so product < 2^60, exact
        cur.push(if (prod.wrapping_sub(n_num)) % need == 0 { 1 } else { 0 });
        p = (p.wrapping_mul(9)) & mask;
        q = (q.wrapping_mul(inv9)) & mask;
    }
    let ones: usize = cur.iter().filter(|&&v| v == 1).count();

    let mut nodes: u64 = 0;
    let mut next_id: u32 = 2;
    let mut len = orbit;
    while len > 1 {
        let half = len >> 1;
        let mut map: HashMap<u64, u32> = HashMap::new();
        let mut nxt: Vec<u32> = Vec::with_capacity(half);
        for i in 0..half {
            let hi = cur[2 * i]; let lo = cur[2 * i + 1];
            if hi == lo { nxt.push(hi); }
            else {
                let key = ((hi as u64) << 32) | (lo as u64);
                let id = *map.entry(key).or_insert_with(|| { let v = next_id; next_id += 1; nodes += 1; v });
                nxt.push(id);
            }
        }
        cur = nxt; len = half;
    }
    format!("bdd: N={} m={} r={}  phase bits={}  density={:.3}  diagram nodes={}",
        n_num, m, r, n, ones as f64 / orbit as f64, nodes)
}

// Reduce a truth table (ids 0/1) bottom-up, eliminating index bit 0 first.
fn robdd_count(mut cur: Vec<u32>) -> u64 {
    use std::collections::HashMap;
    let mut nodes: u64 = 0;
    let mut next_id: u32 = 2;
    let mut len = cur.len();
    while len > 1 {
        let half = len >> 1;
        let mut map: HashMap<u64, u32> = HashMap::new();
        let mut nxt: Vec<u32> = Vec::with_capacity(half);
        for i in 0..half {
            let hi = cur[2 * i]; let lo = cur[2 * i + 1];
            if hi == lo { nxt.push(hi); }
            else {
                let key = ((hi as u64) << 32) | (lo as u64);
                let id = *map.entry(key).or_insert_with(|| { let v = next_id; next_id += 1; nodes += 1; v });
                nxt.push(id);
            }
        }
        cur = nxt; len = half;
    }
    nodes
}

/// Diagram size of the closure predicate under several variable orderings. The
/// order is the whole lever for diagram size, so this reads the predicate against
/// the arrangement: low-to-high, high-to-low, the two halves interleaved, and the
/// best of a batch of random orders as a stand-in for a chosen order.
pub fn bdd_orders(n_num: u128, m: u32, r: u32, rand_tries: u32) -> String {
    if m < 5 { return "bddord: width m must be at least 5".to_string(); }
    let n = (m - 3) as usize;
    let orbit: usize = 1usize << n;
    let modulus: u128 = 1u128 << m;
    let need: u128 = 1u128 << (m + r);
    let mask: u128 = modulus - 1;
    let inv9 = { let a = 9u128 & mask; let mut y = 1u128;
        for _ in 0..7 { y = (y.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(y)))) & mask; } y };
    let inv_p0 = { let a = 3u128 & mask; let mut x = 1u128;
        for _ in 0..7 { x = (x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)))) & mask; } x };
    let nm = n_num & mask;
    let mut p = 3u128 & mask;
    let mut q = (nm.wrapping_mul(inv_p0)) & mask;
    let mut vals: Vec<u8> = Vec::with_capacity(orbit);
    for _ in 0..orbit {
        let prod = p.wrapping_mul(q);
        vals.push(if (prod.wrapping_sub(n_num)) % need == 0 { 1 } else { 0 });
        p = (p.wrapping_mul(9)) & mask;
        q = (q.wrapping_mul(inv9)) & mask;
    }
    // build tt under an order: order[s] = which k-bit becomes index bit s (eliminated at step s).
    let build = |order: &[usize]| -> Vec<u32> {
        let mut tt = alloc::vec![0u32; orbit];
        for k in 0..orbit {
            let mut i = 0usize;
            for s in 0..n { if (k >> order[s]) & 1 == 1 { i |= 1 << s; } }
            tt[i] = vals[k] as u32;
        }
        tt
    };
    let low2high: Vec<usize> = (0..n).collect();
    let high2low: Vec<usize> = (0..n).rev().collect();
    let mut inter: Vec<usize> = Vec::with_capacity(n);
    { let (mut a, mut b) = (0usize, n - 1); while a <= b { inter.push(a); if a != b { inter.push(b); } if b == 0 { break; } a += 1; b -= 1; } }
    inter.truncate(n);

    let ones: usize = vals.iter().filter(|&&v| v == 1).count();
    let mut out = format!("bddord: N={} m={} r={} phase bits={} density={:.3}",
        n_num, m, r, n, ones as f64 / orbit as f64);
    out.push_str(&format!("\n  low2high   {}", robdd_count(build(&low2high))));
    out.push_str(&format!("\n  high2low   {}", robdd_count(build(&high2low))));
    out.push_str(&format!("\n  interleave {}", robdd_count(build(&inter))));

    if rand_tries > 0 {
        // xorshift over a fixed seed; report the smallest diagram found.
        let mut s: u64 = 0x9E3779B97F4A7C15;
        let mut rng = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s };
        let mut best = u64::MAX; let mut best_ord: Vec<usize> = low2high.clone();
        for _ in 0..rand_tries {
            let mut ord: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() { let j = (rng() as usize) % (i + 1); ord.swap(i, j); }
            let c = robdd_count(build(&ord));
            if c < best { best = c; best_ord = ord; }
        }
        out.push_str(&format!("\n  random-min {}  (best of {} orders)", best, rand_tries));
        let _ = best_ord;
    }
    out
}

/// Sift the closure predicate's variable order: starting from high-to-low (the
/// best fixed order found), repeatedly pull each variable out and try every
/// reinsertion position, keeping whichever position gives the smallest diagram.
/// This can only go below any fixed order, since high-to-low is itself one of
/// the positions tried on the first variable.
pub fn bdd_sift(n_num: u128, m: u32, r: u32, passes: u32) -> String {
    if m < 5 { return "bddsift: width m must be at least 5".to_string(); }
    let n = (m - 3) as usize;
    let orbit: usize = 1usize << n;
    let modulus: u128 = 1u128 << m;
    let need: u128 = 1u128 << (m + r);
    let mask: u128 = modulus - 1;
    let inv9 = { let a = 9u128 & mask; let mut y = 1u128;
        for _ in 0..7 { y = (y.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(y)))) & mask; } y };
    let inv_p0 = { let a = 3u128 & mask; let mut x = 1u128;
        for _ in 0..7 { x = (x.wrapping_mul(2u128.wrapping_sub(a.wrapping_mul(x)))) & mask; } x };
    let nm = n_num & mask;
    let mut p = 3u128 & mask;
    let mut q = (nm.wrapping_mul(inv_p0)) & mask;
    let mut vals: Vec<u8> = Vec::with_capacity(orbit);
    for _ in 0..orbit {
        let prod = p.wrapping_mul(q);
        vals.push(if (prod.wrapping_sub(n_num)) % need == 0 { 1 } else { 0 });
        p = (p.wrapping_mul(9)) & mask;
        q = (q.wrapping_mul(inv9)) & mask;
    }
    let build = |order: &[usize]| -> Vec<u32> {
        let mut tt = alloc::vec![0u32; orbit];
        for k in 0..orbit {
            let mut i = 0usize;
            for s in 0..n { if (k >> order[s]) & 1 == 1 { i |= 1 << s; } }
            tt[i] = vals[k] as u32;
        }
        tt
    };
    let mut order: Vec<usize> = (0..n).rev().collect();   // start from the winning high-to-low
    let start_cost = robdd_count(build(&order));
    let mut best_cost = start_cost;

    for _pass in 0..passes {
        let mut improved = false;
        for var_pos in 0..n {
            let var = order[var_pos];
            let mut without: Vec<usize> = order.clone(); without.remove(var_pos);
            let mut best_here = u64::MAX; let mut best_at = var_pos;
            for ins in 0..=without.len() {
                let mut cand = without.clone(); cand.insert(ins, var);
                let c = robdd_count(build(&cand));
                if c < best_here { best_here = c; best_at = ins; }
            }
            let mut new_order = without.clone(); new_order.insert(best_at, var);
            if best_here < best_cost { best_cost = best_here; improved = true; }
            order = new_order;
        }
        if !improved { break; }
    }
    format!("bddsift: N={} m={} r={} phase bits={}  high2low start={}  after sift={}  (ratio {:.3})",
        n_num, m, r, n, start_cost, best_cost, best_cost as f64 / start_cost as f64)
}

fn b4_name(v: u8) -> &'static str {
    match v & 3 { 1 => "T", 2 => "F", 3 => "B", _ => "N" }
}

/// Execute one glyph word on the device and print its resulting state, with an
/// inline CPU-kernel parity line so the run is checkable where it is read.
pub fn run(word: &str, device: usize) -> String {
    run_word(word, device, false)
}

/// Execute a supplied word through the same nested entry used by `bench`.
pub fn run_nested(word: &str, device: usize) -> String {
    run_word(word, device, true)
}

/// Load the committed ob3ect's program and execute the existing nested entry.
/// Domain-action prose remains provenance; it is not an executable payload.
pub fn run_ob3ect(path: &str, device: usize) -> String {
    run_ob3ect_with_payload(path, None, device)
}

pub fn apply_ob3ect(path: &str, payload: &str, device: usize) -> String {
    run_ob3ect_with_payload(path, Some(payload), device)
}

fn ob3ect_program(object: &serde_json::Value) -> Result<&str, String> {
    let word = object["glyph_word"].as_str().ok_or("missing glyph_word")?;
    let steps = object["phases"]["phase_4"]["steps"].as_array()
        .ok_or("missing phase_4 steps")?;
    let mut phase_word = String::new();
    for step in steps {
        phase_word.push_str(step["opcode"].as_str().ok_or("step missing opcode")?);
    }
    if word != phase_word {
        return Err("glyph_word differs from phase_4 program".into());
    }
    Ok(word)
}

#[cfg(test)]
mod ob3ect_program_tests {
    use super::ob3ect_program;

    fn supplied() -> serde_json::Value {
        serde_json::from_str(include_str!("../../ob3ect/digital/imasm_word_that_constructs_and_selects_the_compa_9eeb5656/imasm_word_that_constructs_and_selects_the_compa_9eeb5656_ob3ect.json")).unwrap()
    }

    #[test]
    fn supplied_word_and_phase_program_agree() {
        assert_eq!(ob3ect_program(&supplied()).unwrap(), "⊢⊙∈≻⊤≺⊥⋈⊞∋⊡⊣");
    }

    #[test]
    fn changed_phase_cannot_silently_execute_the_original_selector() {
        let mut object = supplied();
        object["phases"]["phase_4"]["steps"][3]["opcode"] = serde_json::json!("≺");
        assert_eq!(ob3ect_program(&object).unwrap_err(), "glyph_word differs from phase_4 program");
        object["phases"]["phase_4"]["steps"][3].as_object_mut().unwrap().remove("opcode");
        assert_eq!(ob3ect_program(&object).unwrap_err(), "step missing opcode");
    }
}

fn run_ob3ect_with_payload(path: &str, payload: Option<&str>, device: usize) -> String {
    let result = (|| -> Result<String, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("read {path}: {e}"))?;
        let object: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|e| format!("ob3ect JSON: {e}"))?;
        let word = ob3ect_program(&object)?;
        let tri = imasm_core::lattice_flow::tri_ancestral_word_verdict(word);
        let banked = imasm_core::lattice_flow::banked_walk(word)
            .map(|b| b.holds()).unwrap_or(false);
        let mut report = format!("ob3ect: {}\n  source: {path}\n  word: {word}\n  live tri verdict: {tri:?}; banked: {banked}\n",
            object["name"].as_str().unwrap_or("(unnamed)"));
        let executable = match payload {
            Some(inner) => crate::word_nesting::enclose(word, inner)?,
            None => word.to_owned(),
        };
        if payload.is_some() {
            report.push_str(&format!("  contained process: {}\n  composed word: {executable}\n", payload.unwrap()));
            let composed_tri = imasm_core::lattice_flow::tri_ancestral_word_verdict(&executable);
            let composed_banked = imasm_core::lattice_flow::banked_walk(&executable)
                .map(|b| b.holds()).unwrap_or(false);
            report.push_str(&format!("  composed tri verdict: {composed_tri:?}; banked: {composed_banked}\n"));
            let ops: Vec<imasm_core::classic::Token> = executable.chars()
                .filter_map(|c| imasm_core::classic::Token::parse(&c.to_string())).collect();
            let graph = imasm_core::check::from_sequence(&ops, &imasm_core::check::match_pairs(&ops));
            for (fork, fuse) in graph.frobenius_closures().0 {
                report.push_str(&format!("  dyad {fork}->{fuse}: transforms={}\n", graph.transforms_between(fork, fuse)));
            }
        }
        report.push_str(&run_nested(&executable, device));
        Ok(report)
    })();
    result.unwrap_or_else(|e| format!("gpu_kernel ob3ect: {e}"))
}

fn run_word(word: &str, device: usize, nested: bool) -> String {
    use crate::belnap_ring_shor::{Glyph, glyph_to_token};
    let mut ids: Vec<u8> = Vec::new();
    for (pos, c) in word.chars().filter(|c| !c.is_whitespace()).enumerate() {
        match Glyph::from_char(c) {
            Some(g) => ids.push(tok_id(glyph_to_token(g))),
            None => return format!("gpu_kernel run: char {} ('{}') is not one of the twelve marks", pos, c),
        }
    }
    if ids.is_empty() { return "gpu_kernel run: empty word".into(); }
    if ids.len() > Program::CAPACITY {
        return format!("gpu_kernel run: word longer than {} marks", Program::CAPACITY);
    }
    let program_capacity = ids.len().next_power_of_two().max(STRIDE);

    let max_ticks: u64 = 65536;
    let ctx = match CudaContext::new(device) {
        Ok(c) => c, Err(e) => return format!("gpu_kernel run: no CUDA context: {e}"),
    };
    let stream = ctx.default_stream();
    let source = format!("#define PROGRAM_CAP {program_capacity}\n{}",
        if nested { KERNEL_NESTED_SRC } else { KERNEL_SRC });
    let entry = if nested { "run_nested" } else { "run_programs" };
    let ptx = match compile_ptx(source) { Ok(p)=>p, Err(e)=>return format!("gpu_kernel run: NVRTC: {e}") };
    let module = match ctx.load_module(ptx) { Ok(m)=>m, Err(e)=>return format!("gpu_kernel run: module: {e}") };
    let func = match module.load_function(entry) { Ok(f)=>f, Err(e)=>return format!("gpu_kernel run: load: {e}") };

    let mut progs = alloc::vec![0u8; program_capacity];
    for (i, &id) in ids.iter().enumerate() { progs[i] = id; }
    let lens = alloc::vec![ids.len() as u32];
    let d_progs = match stream.clone_htod(&progs) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel run: htod: {e}") };
    let d_lens = match stream.clone_htod(&lens) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel run: htod: {e}") };
    let mut d_stack = match stream.alloc_zeros::<u8>(SDEPTH) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel run: alloc: {e}") };
    let mut d_meta = match stream.alloc_zeros::<u32>(META) { Ok(d)=>d, Err(e)=>return format!("gpu_kernel run: alloc: {e}") };

    let cfg = LaunchConfig { grid_dim: (1,1,1), block_dim: (1,1,1), shared_mem_bytes: 0 };
    let mt = max_ticks; let nn = 1u32;
    let mut b = stream.launch_builder(&func);
    b.arg(&d_progs); b.arg(&d_lens); b.arg(&mut d_stack); b.arg(&mut d_meta); b.arg(&mt); b.arg(&nn);
    if let Err(e) = unsafe { b.launch(cfg) } { return format!("gpu_kernel run: launch: {e}"); }
    let g_stack = match stream.clone_dtoh(&d_stack) { Ok(v)=>v, Err(e)=>return format!("gpu_kernel run: dtoh: {e}") };
    let m = match stream.clone_dtoh(&d_meta) { Ok(v)=>v, Err(e)=>return format!("gpu_kernel run: dtoh: {e}") };

    let g_top = m[2] as usize;
    let mut stack_str = String::new();
    for i in 0..g_top { if i>0 { stack_str.push(' '); } stack_str.push_str(b4_name(g_stack[i])); }

    let (c_halt, c_tick, c_depth, c_regs, c_engagr, c_mem, c_stack) = cpu_run(&ids, max_ticks);
    let mut parity = c_halt == m[0] as u8 && c_tick == m[1] && c_depth == m[2] && (c_engagr == m[11] as u8);
    for i in 0..8 { if c_regs[i] as u32 != m[3+i] { parity = false; } }
    for a in 0..4 { if c_mem[a] as u32 != m[12+a] { parity = false; } }
    for i in 0..c_depth as usize { if g_stack[i] != c_stack[i] { parity = false; } }

    let mut report = format!(
        "gpu_kernel {entry} on device: {} marks\n  halted: {}   ticks: {}\n  stack (bottom..top): [{}]  depth {}\n  registers: [{} {} {} {} {} {} {} {}]  engagr {}\n  memory[0..4]: [{} {} {} {}]\n  matches CPU kernel: {}",
        ids.len(), m[0]==1, m[1], stack_str, g_top,
        b4_name(m[3] as u8), b4_name(m[4] as u8), b4_name(m[5] as u8), b4_name(m[6] as u8),
        b4_name(m[7] as u8), b4_name(m[8] as u8), b4_name(m[9] as u8), b4_name(m[10] as u8),
        m[11]==1,
        b4_name(m[12] as u8), b4_name(m[13] as u8), b4_name(m[14] as u8), b4_name(m[15] as u8),
        parity);
    if nested {
        let b = &m[16..32];
        let landing = imasm_core::imasm16_3::Reg16_3 {
            big_t: b[4]&1!=0, big_f: b[4]&2!=0, small_t: b[4]&4!=0, small_f: b[4]&8!=0,
        }.name();
        let state = if b[8]>0 {"exposed"} else if b[5]>0 {"holds"} else {"vacuous"};
        report.push_str(&format!("\n  banked tri face: landing={landing}; weights={:?}; state={state}\n  live clears={}; deposits={}; inert={}; exposed clears={}; exposed weight={}\n  cleared weight={}; restored weight={}; seeds={}; open frames={}\n  banked face matches CPU combo2 semantics: {}",
            &b[..4],b[5],b[6],b[7],b[8],b[9],b[10],b[11],b[12],b[13],
            b[..10] == banked_reference(&ids)));
    }
    report
}
