#ifdef IMASM_FACTOR_RELATION
// Tensor-core floor of the relation tower. One warp carries one circuit
// operator as a one-hot member of the sixteen-state IMASM carrier through
// `depth` identity morphisms. Identity is intentional here: nesting transports
// an operator without changing which operator it is, while mma_sync performs
// each enclosing morphism as real tensor work.
extern "C" __global__ void run_tensor_enfold_ops(
    const unsigned int *ops,unsigned int count,unsigned int depth,unsigned int *enfolded)
{
    using namespace nvcuda;
    unsigned int op_index=blockIdx.x;
    unsigned int lane=threadIdx.x&31u;
    if(op_index>=count||threadIdx.x>=32) return;
    __shared__ half identity[256],input[256];
    __shared__ float product[256];
    __shared__ unsigned int live;
    if(lane==0) live=ops[4*op_index]&15u;
    __syncwarp();
    for(unsigned int level=0;level<depth;level++) {
        for(unsigned int i=lane;i<256;i+=32) {
            unsigned int row=i>>4,col=i&15u;
            identity[i]=__float2half(row==col?1.0f:0.0f);
            input[i]=__float2half(row==live?1.0f:0.0f);
        }
        __syncwarp();
        wmma::fragment<wmma::matrix_a,16,16,16,half,wmma::row_major> a;
        wmma::fragment<wmma::matrix_b,16,16,16,half,wmma::row_major> b;
        wmma::fragment<wmma::accumulator,16,16,16,float> c;
        wmma::load_matrix_sync(a,identity,16);wmma::load_matrix_sync(b,input,16);
        wmma::fill_fragment(c,0.0f);wmma::mma_sync(c,a,b,c);
        wmma::store_matrix_sync(product,c,16,wmma::mem_row_major);
        __syncwarp();
        if(lane==0) for(unsigned int row=0;row<16;row++)
            if(product[16*row]>0.5f) live=row;
        __syncwarp();
    }
    if(lane==0) {
        enfolded[4*op_index]=live;
        enfolded[4*op_index+1]=ops[4*op_index+1];
        enfolded[4*op_index+2]=ops[4*op_index+2];
        enfolded[4*op_index+3]=ops[4*op_index+3];
    }
}
// Reduced ordered decision graphs with complemented edges. Apply traverses
// pairs of graph nodes and interns their reconnections; it never constructs
// or tests a prospective numeric factor pair.
struct RFrame { unsigned int a,b,v,ah,bh,low,stage; };
struct RGraph {
    unsigned int *nodes,*unique,*cache,cap,used,error;
    unsigned long long steps,limit;
    __device__ unsigned int hash(unsigned int a,unsigned int b,unsigned int c) {
        unsigned int x=a*0x9e3779b9u ^ b*0x85ebca6bu ^ c*0xc2b2ae35u;
        x^=x>>16; x*=0x7feb352du; x^=x>>15; return x;
    }
    __device__ unsigned int var(unsigned int edge) {
        return edge<2?0xffffffffu:nodes[(edge>>1)*3];
    }
    __device__ unsigned int make(unsigned int v,unsigned int low,unsigned int high) {
        if(low==high) return low;
        unsigned int flip=high&1u;
        low^=flip; high^=flip;
        unsigned int slot=hash(v,low,high)&(2*cap-1);
        while(unique[slot]) {
            unsigned int node=unique[slot];
            if(nodes[3*node]==v && nodes[3*node+1]==low && nodes[3*node+2]==high)
                return (node<<1)^flip;
            slot=(slot+1)&(2*cap-1);
        }
        if(used>=cap) {error=1;return 0;}
        unsigned int node=used++;
        nodes[3*node]=v; nodes[3*node+1]=low; nodes[3*node+2]=high;
        unique[slot]=node;
        return (node<<1)^flip;
    }
    __device__ unsigned int meet(unsigned int a,unsigned int b) {
        RFrame stack[128]; int top=0; unsigned int result=0;
        stack[0].a=a;stack[0].b=b;stack[0].stage=0;
        while(top>=0 && !error) {
            if(++steps>limit) {error=2;break;}
            RFrame &f=stack[top];
            if(f.stage==0) {
                if(f.a>f.b) {unsigned int t=f.a;f.a=f.b;f.b=t;}
                if(f.a==0 || f.a==(f.b^1u)) {result=0;--top;continue;}
                if(f.a==1 || f.a==f.b) {result=f.b;--top;continue;}
                unsigned int slot=(hash(f.a,f.b,0)&(2*cap-1))*3;
                if(cache[slot]==f.a && cache[slot+1]==f.b) {result=cache[slot+2];--top;continue;}
                unsigned int va=var(f.a),vb=var(f.b);
                f.v=va<vb?va:vb;
                unsigned int al=f.a,bl=f.b; f.ah=f.a;f.bh=f.b;
                if(va==f.v) {al=nodes[(f.a>>1)*3+1]^(f.a&1);f.ah=nodes[(f.a>>1)*3+2]^(f.a&1);}
                if(vb==f.v) {bl=nodes[(f.b>>1)*3+1]^(f.b&1);f.bh=nodes[(f.b>>1)*3+2]^(f.b&1);}
                f.stage=1;
                if(top==127) {error=3;break;}
                ++top;stack[top].a=al;stack[top].b=bl;stack[top].stage=0;
            } else if(f.stage==1) {
                f.low=result;f.stage=2;
                unsigned int ah=f.ah,bh=f.bh;
                ++top;stack[top].a=ah;stack[top].b=bh;stack[top].stage=0;
            } else {
                result=make(f.v,f.low,result);
                if(error) break;
                unsigned int slot=(hash(f.a,f.b,0)&(2*cap-1))*3;
                cache[slot]=f.a;cache[slot+1]=f.b;cache[slot+2]=result;
                --top;
            }
        }
        return result;
    }
};

// Carry one IMASM mark through one complete relation protocol. The mark is the
// payload: FSPLIT copies it onto both arms, the working arm traverses the
// protocol, and FFUSE admits the mark only when both arms return the same
// payload. The returned mark is what the relation builder dispatches. Applying
// this repeatedly makes level d consume the mark emitted by level d+1.
__device__ __forceinline__ unsigned int imasm_relation_level(
    unsigned int payload,unsigned long long *ticks)
{
    const unsigned char word[12]={0,8,6,2,5,3,9,4,10,7,11,1};
    unsigned int left=payload,right=payload,live=payload;
    bool split=false;
    for(unsigned int i=0;i<12;i++) {
        unsigned int mark=word[i]; ++*ticks;
        if(mark==0) live=payload;             // VINIT supplies the inner mark
        else if(mark==8) live=live;           // IMSCRIB reads it as program data
        else if(mark==6) {left=live;right=live;split=true;}
        else if(mark==2||mark==5||mark==3||mark==9||mark==4||mark==10)
            left=left;                        // working-arm IMASM transformations
        else if(mark==7 && split) {           // FFUSE requires both readings
            if(left!=right) return 16u;
            live=left; split=false;
        }
        else if(mark==11) live=live;           // IFIX commits the emitted mark
        else if(mark==1) return split?16u:live;// TANCH returns it to the caller
    }
    return 16u;
}

__device__ __forceinline__ unsigned int imasm_relation_tower(
    unsigned int mark,unsigned int depth,unsigned long long *ticks)
{
    unsigned int emitted=mark;
    for(unsigned int level=0;level<depth;level++) {
        emitted=imasm_relation_level(emitted,ticks);
        if(emitted>=16u) break;
    }
    return emitted;
}

// Enfold one temporal construction continuation in the canonical cut of the
// fully-nested phase-based prime-factorization word. FSPLIT banks the cursor;
// the T arm descends through AFWD/EVALT/AREV while the F arm carries the
// composite hierarchy through EVALF/CLINK/IMSCRIB/ENGAGR. FFUSE restores the
// banked cursor, which is fixed, composed, reinscribed, and exposed only by the
// final TANCH. Every operator traverses the requested IMASM tower before acting.
__device__ __forceinline__ unsigned int imasm_enfold_continuation(
    unsigned int payload,unsigned int depth,unsigned long long *ticks)
{
    const unsigned char word[14]={0,6,2,5,3,9,4,8,10,7,11,4,8,1};
    unsigned int live=payload,banked=0,left=0,right=0;
    bool frame=false,fixed=false;
    for(unsigned int i=0;i<14;i++) {
        unsigned int mark=imasm_relation_tower(word[i],depth,ticks);
        if(mark>=16u) return 0xffffffffu;
        if(mark==0) {live=payload;fixed=false;}
        else if(mark==6) {banked=live;left=live;right=live;frame=true;}
        else if(mark==2||mark==5) left=left;
        else if(mark==3) left=0;
        else if(mark==9||mark==4||mark==8||mark==10) right=right;
        else if(mark==7) {
            if(!frame||left!=0||right!=banked) return 0xffffffffu;
            live=banked;frame=false;
        }
        else if(mark==11) fixed=!frame&&live==payload;
        else if(mark==1 && i==13) return fixed&&live==payload?live:0xffffffffu;
    }
    return 0xffffffffu;
}

// One reduced decision node is one nested selector protocol. FSPLIT exposes
// its low and high continuations, EVALT/EVALF read whether those supports are
// inhabited, FFUSE returns the first surviving arm, and IFIX commits the bit
// carried by that arm. Every protocol mark itself crosses the surrounding
// tower before it may act. The chosen edge is therefore emitted by IMASM; the
// witness walk no longer selects a child with a host-language conditional.
__device__ __forceinline__ unsigned int imasm_select_decision_node(
    unsigned int low,unsigned int high,unsigned int nest_depth,
    unsigned int *selected_high,unsigned long long *ticks)
{
    const unsigned char word[8]={0,6,5,9,7,11,8,1};
    unsigned int left=0,right=0,chosen=0; bool split=false,committed=false;
    for(unsigned int i=0;i<8;i++) {
        unsigned int mark=imasm_relation_tower(word[i],nest_depth,ticks);
        if(mark==0) {chosen=0;committed=false;}
        else if(mark==6) {left=low;right=high;split=true;}
        else if(mark==5 && split) left=left?left:0;   // EVALT reads low support
        else if(mark==9 && split) right=right?right:0;// EVALF reads high support
        else if(mark==7 && split) {                  // FFUSE returns a live arm
            if(left) {chosen=left;*selected_high=0;}
            else {chosen=right;*selected_high=1;}
            split=false;
        }
        else if(mark==11) committed=chosen!=0;        // IFIX commits the bit
        else if(mark==8 && committed) chosen=chosen;  // IMSCRIB carries next node
        else if(mark==1) return committed?chosen:0;   // TANCH emits continuation
        else if(mark>=16u) return 0;
    }
    return 0;
}

// Reclaims the node table instead of only ever growing it. Every node the
// make/meet apply recursion ever interned stays in the table forever, but
// most of that graph goes dead well before construction finishes: an early
// column's partial sum, once folded into a later carry, is never read again
// by any surviving wire. This runs strictly between two completed ops (never
// mid-meet, so no in-flight apply frame ever needs remapping): mark every
// node reachable from a wire that some later op (or the final root) still
// reads, sweep the rest, and compact what survives into a dense prefix.
// Correct in one linear backward pass with no stack at all, because make()
// only ever interns a node after both its children already have a slot: a
// child's index is always strictly less than its parent's, so processing
// node indices from used-1 down to 1 already visits every node in the exact
// order needed to propagate a mark down to its children.
// The mark bitset (in cache) and the newidx scratch (in unique) both need a
// clean slate on entry, and cache needs one again on exit to serve as the
// apply memo table a resumed construction expects. Zeroing capacity-sized
// buffers has no dependency structure at all -- the host does it with a
// hardware memset (see select_relation) instead of a thread looping over
// millions of words one at a time; run_compact starts from an already-zeroed
// cache and unique, and select_relation zeroes cache again after this and
// unique again before run_compact_rehash.
extern "C" __global__ void run_compact(
    unsigned int *nodes,unsigned int *unique,unsigned int *cache,unsigned int *values,
    const unsigned int *last_use,unsigned int root,unsigned int current_i,
    unsigned int used,unsigned int capacity,unsigned int nest_depth,
    unsigned long long *out)
{
    if(blockIdx.x||threadIdx.x) return;
    // IFIX is the only mark that may commit the live relation into a new dense
    // node prefix. The physical rewrite cannot begin until the nested tower
    // emits IFIX unchanged.
    if(imasm_enfold_continuation(11u,nest_depth,&out[11])!=11u) {out[0]=5;return;}
    for(unsigned int k=0;k<current_i;k++) {
        bool live=(last_use[k]>=current_i)||(k==root);
        if(!live) continue;
        unsigned int e=values[k];
        if(e>=2) { unsigned int n=e>>1; cache[n>>5]|=1u<<(n&31); }
    }
    for(unsigned int node=used-1;node>=1;node--) {
        if(cache[node>>5]&(1u<<(node&31))) {
            unsigned int lo=nodes[3*node+1],hi=nodes[3*node+2];
            if(lo>=2) { unsigned int n=lo>>1; cache[n>>5]|=1u<<(n&31); }
            if(hi>=2) { unsigned int n=hi>>1; cache[n>>5]|=1u<<(n&31); }
        }
        if(node==1) break;
    }
    // newidx lives in unique[] until the hash table is rebuilt below.
    unique[0]=0;
    unsigned int running=1;
    for(unsigned int node=1;node<used;node++) {
        bool m=(cache[node>>5]&(1u<<(node&31)))!=0;
        unique[node]=m?running++:0xFFFFFFFFu;
    }
    for(unsigned int node=1;node<used;node++) {
        if(unique[node]==0xFFFFFFFFu) continue;
        unsigned int lo=nodes[3*node+1],hi=nodes[3*node+2];
        unsigned int nlo=lo<2?lo:((unique[lo>>1]<<1)|(lo&1));
        unsigned int nhi=hi<2?hi:((unique[hi>>1]<<1)|(hi&1));
        unsigned int dst=unique[node];
        nodes[3*dst]=nodes[3*node]; nodes[3*dst+1]=nlo; nodes[3*dst+2]=nhi;
    }
    for(unsigned int k=0;k<current_i;k++) {
        unsigned int e=values[k];
        if(e<2) continue;
        unsigned int nn=unique[e>>1];
        if(nn!=0xFFFFFFFFu) values[k]=(nn<<1)|(e&1);
    }
    // care (out[10]) is a copy of some earlier values[k], not itself a slot
    // in values[]; it needs the same remap or a resumed run reads a stale
    // node index straight out of a register-sized field with no array
    // indirection to have already fixed it.
    {
        unsigned int e=(unsigned int)out[10];
        if(e>=2) { unsigned int nn=unique[e>>1]; if(nn!=0xFFFFFFFFu) out[10]=(unsigned long long)((nn<<1)|(e&1u)); }
    }
    out[0]=3; out[3]=running-1;
}

// Rebuilds the hash table from the compacted, dense node prefix run_compact
// just produced. Split out because unique[] needs a host memset between
// (it was newidx scratch a moment ago) and this is otherwise the same
// single-thread linear-probe insert the resume-after-growth path already
// does elsewhere in run_nested.
extern "C" __global__ void run_compact_rehash(
    const unsigned int *nodes,unsigned int *unique,unsigned int running,
    unsigned int capacity,unsigned int nest_depth,unsigned long long *out)
{
    if(blockIdx.x||threadIdx.x) return;
    // CLINK is the relation's composition mark. Rebuilding the unique table is
    // a composition of each surviving node with its canonical table address.
    if(imasm_enfold_continuation(4u,nest_depth,&out[11])!=4u) {out[0]=5;return;}
    RGraph g; g.cap=capacity;
    for(unsigned int node=1;node<running;node++) {
        unsigned int slot=g.hash(nodes[3*node],nodes[3*node+1],nodes[3*node+2])&(2*capacity-1);
        while(unique[slot]) slot=(slot+1)&(2*capacity-1);
        unique[slot]=node;
    }
}

// The host owns CUDA allocation, but allocation is not allowed to advance the
// relation by itself. AFWD must traverse the same nested IMASM tower first;
// out[12] is the explicit permission read by the host continuation.
extern "C" __global__ void run_relation_control(
    unsigned int mark,unsigned int nest_depth,unsigned long long *out)
{
    if(blockIdx.x||threadIdx.x) return;
    out[12]=imasm_enfold_continuation(mark,nest_depth,&out[11]);
}

extern "C" __global__ void run_relation_epoch(
    unsigned int continuation,unsigned int nest_depth,unsigned long long *out)
{
    if(blockIdx.x||threadIdx.x) return;
    out[12]=imasm_enfold_continuation(continuation,nest_depth,&out[11]);
}

// Emit one temporal factor limb. The caller supplies the continuation edge
// returned by the previous tick. This invocation walks only decisions whose
// factor-bit position belongs to limb_index, folds every choice through the
// nested selector protocol, emits two bounded 32-bit limb values, and returns
// the untouched next edge. Repeating the same morphism makes the time needed
// for an arbitrarily long witness without embedding that witness in one device
// register.
extern "C" __global__ void run_witness_limb(
    const unsigned int *nodes,unsigned int edge,unsigned int limb_index,
    unsigned int nest_depth,unsigned long long *out)
{
    if(blockIdx.x||threadIdx.x) return;
    unsigned int p=0,q=0,path=0;
    unsigned int first=32u*limb_index,last=first+32u;
    unsigned long long ticks=0,vessel_ticks=0;
    edge=imasm_enfold_continuation(edge,nest_depth,&vessel_ticks);
    if(edge==0xffffffffu) {out[0]=edge;out[1]=out[2]=out[3]=out[4]=out[5]=0;return;}
    while(edge>=2) {
        unsigned int node=edge>>1,bit=nodes[3*node],factor_bit=bit>>1;
        if(factor_bit>=last) break;
        unsigned int low=nodes[3*node+1]^(edge&1);
        unsigned int high=nodes[3*node+2]^(edge&1),selected_high=0;
        edge=imasm_select_decision_node(low,high,nest_depth,&selected_high,&ticks);
        if(selected_high && factor_bit>=first) {
            unsigned int local=factor_bit-first;
            if(bit&1u) q|=1u<<local; else p|=1u<<local;
        }
        ++path;
    }
    out[0]=edge;out[1]=p;out[2]=q;out[3]=path;
    out[4]=ticks+vessel_ticks;out[5]=ticks;
}

extern "C" __global__ void run_nested(
    const unsigned char *word,unsigned int word_len,
    const unsigned int *ops,unsigned int count,unsigned int root,unsigned int width,
    unsigned int *nodes,unsigned int *unique,unsigned int *cache,unsigned int *values,
    unsigned int capacity,unsigned int nest_depth,unsigned int epoch_ops,
    unsigned long long step_limit,unsigned long long *out)
{
    if(blockIdx.x || threadIdx.x) return;
    RGraph g;g.nodes=nodes;g.unique=unique;g.cache=cache;g.cap=capacity;
    bool grow_resume=out[0]==2;
    bool compact_resume=out[0]==3;
    bool temporal_resume=out[0]==6;
    bool resume=grow_resume||compact_resume||temporal_resume;
    g.used=resume?(unsigned int)out[3]+1:1;
    g.error=0;g.steps=resume?out[4]:0;g.limit=step_limit;
    unsigned int begin=resume?(unsigned int)out[9]:0;
    unsigned int initial_care=resume?(unsigned int)out[10]:1;
    if(grow_resume) {
        // The node pool and circuit values survived host allocation growth.
        // Reindex the same nodes in the larger table; no relation is rebuilt.
        for(unsigned int i=1;i<g.used;i++) {
            unsigned int slot=g.hash(nodes[3*i],nodes[3*i+1],nodes[3*i+2])&(2*capacity-1);
            while(unique[slot]) slot=(slot+1)&(2*capacity-1);
            unique[slot]=i;
        }
    }
    // A compact_resume already leaves unique[] fully rebuilt for the
    // unchanged capacity; nothing to reindex.
    out[1]=out[2]=out[5]=out[8]=0;
    unsigned int live=0,held=0; bool opened=false,constructed=false,closed=false;
    // The outer process holds the relation while its working arm constructs
    // the circuit's symbolic support. Fixation reads one path from the final
    // reduced relation only after that relation has reconnected at the fuse.
    for(unsigned int ip=0;ip<word_len && !g.error;ip++) {
        switch(word[ip]) {
            case 0: live=held=0;opened=constructed=closed=false;break;
            case 6: opened=true;break;
            case 3: live=0;break;
            case 4:
                if(!opened) {g.error=4;break;}
                {
                unsigned int care=initial_care;
                unsigned int epoch_count=0;
                for(unsigned int i=begin;i<count && !g.error && care;i++) {
                    out[9]=i;out[10]=care;
                    const unsigned int *op=ops+4*i;
                    unsigned int emitted=imasm_relation_tower(op[0],nest_depth,&out[11]);
                    switch(emitted) {
                        case 0: values[i]=0;break;
                        case 5: values[i]=1;break;
                        case 8: values[i]=g.make(op[1],0,1);break;
                        case 3: values[i]=values[op[1]]^1u;break;
                        case 4: values[i]=g.meet(values[op[1]],values[op[2]]);break;
                        case 7: values[i]=g.meet(values[op[1]]^1u,values[op[2]]^1u)^1u;break;
                        default: g.error=4;break;
                    }
                    if(!g.error) {
                        values[i]=g.meet(values[i],care);
                        if(op[3]) care=values[i];
                        if(++epoch_count>=epoch_ops && i+1<count) {
                            out[9]=i+1;out[10]=care;
                            out[0]=6;out[3]=g.used-1;out[4]=g.steps;
                            out[6]=care;out[7]=count;
                            return;
                        }
                    }
                }
                if(!g.error) {live=care?values[root]:0;held=live;constructed=true;}
                }
                break;
            case 7: if(opened && constructed) {live=held;closed=true;}opened=false;break;
            case 11: break; // IFIX commits the relation; limbs emit after closure.
        }
    }
    // 0 complete/empty, 1 complete/nonempty, >=2 explicit construction failure.
    if(!g.error && closed && width<=31u) {
        // All children precede their interned parent. Reuse the apply cache
        // after construction to count the complete relation, including free
        // variables and complemented edges. This also checks more than a
        // single extracted witness in the independent controls.
        unsigned long long *counts=(unsigned long long*)cache;
        for(unsigned int i=1;i<g.used;i++) {
            unsigned int v=nodes[3*i];
            unsigned long long total=0;
            for(unsigned int side=1;side<=2;side++) {
                unsigned int edge=nodes[3*i+side];
                unsigned int child_v=edge<2?2*width:g.var(edge);
                unsigned long long amount=edge<2?edge:counts[edge>>1];
                if(edge>=2 && (edge&1)) amount=(1ULL<<(2*width-child_v))-amount;
                total+=amount<<(child_v-v-1);
            }
            counts[i]=total;
        }
        unsigned int v=live<2?2*width:g.var(live);
        unsigned long long amount=live<2?live:counts[live>>1];
        if(live>=2 && (live&1)) amount=(1ULL<<(2*width-v))-amount;
        out[8]=amount<<v;
    }
    else if(!g.error && closed) out[8]=0xffffffffffffffffULL;
    out[0]=g.error?g.error+1:(closed && live?1:0);
    out[3]=g.used-1;out[4]=g.steps;out[6]=live;out[7]=count;
}
#endif
