# IMASM-native compute: the machine inside the twelve

The disassembler and its inverse are written in IMASM, executed by the parasm VM
over the crystal filesystem. No non-IMASM part. Disassembling the tool yields
IMASM, so the round trip can be identity rather than an approximation. This is
the Replicating Code: a program that is its own structural type, a fixed point
of its own compile loop.

## The machine

The substrate is already here. `parasm` (src/parasm.rs) is a register machine
whose native value is B4 {N, T, F, B} and whose operations realize the twelve:

- FSPLIT ∈, FFUSE ∋, ENGAGR ⊞, IFIX ⊡ are the Frobenius core, verbatim.
- A data-dependent branch (JT/JF/JB/JN) is FSPLIT opening arms with EVALT/EVALF
  selecting one. The condition is read off a register's Belnap value.
- A loop is ROTAT: the program counter wraps the word ring (`step` resets pc to
  0 and counts a cycle), the cyclic shift on the whole word.
- CALL/RET is CLINK: composition with a return, the call stack holding the seam.
- MOVE, PUSH, POP, CLEAR are the register plumbing the twelve act on, not a
  thirteenth opcode.
- READ is VINIT reading the boundary in; EMIT is TANCH writing it out.

The crystal filesystem (src/crystal.rs) is the unbounded addressable store. A
register machine with data-dependent branching plus unbounded memory is
universal, so the grammar has no outside to compute in: the machine is the
twelve made operational, and its tape is the crystal.

## The alphabet is B4

The machine's native symbol is B4, four values, two bits. A byte is four cells;
a 256-entry space is four cells deep. A binary is not fed as opaque bytes, it is
re-expressed in the native alphabet as a stream of B4 cells (base-4, the
SIXTEEN_3 carrier one layer up). Input, computation, and output all live in the
same alphabet, which is what lets the tool read itself.

## The decode table is the word, not data

The load-bearing move. A decode step does not look up a table in memory. It
DISPATCHES through a control-flow trie: read a cell, fork on its B4 value, and
the arm you land on is the decoded token. Four cells of nested FSPLIT dispatch
is a full 256-way byte decoder, and it is pure structure. The computation lives
in the SHAPE of the word, which is precisely what "compute inside the twelve"
means. There is nothing to carry as surface payload because the table has become
topology.

## Proven (the first plank)

`parasm::tests::test_imasm_native_decode_step` (src/parasm.rs) is the seed,
running under `cargo test --features hosted`:

- input stream, in the native alphabet: `[T, F, B, N]`
- a decode subroutine (CALL = CLINK) that forks on the cell (JT/JF/JB, the N case
  by fall-through) and emits a TRANSFORMED token, the permutation
  `N→T, T→F, F→B, B→N`
- output: `[F, B, N, T]`

It is a nontrivial function of the input, not an echo, and the decode carries no
data table: the dispatch trie is the table. This is IMASM-native compute
executing on the substrate.

## Proven (the second plank): a full byte decoder

`parasm::tests::test_imasm_full_byte_decoder` widens the trie to a whole byte.
A byte is four cells: two are the OPCODE field, two are the OPERAND.

- The opcode field drives a two-level, 16-leaf dispatch trie (the twelve IMASM
  opcodes plus four spares). The wire opcode encoding is scrambled and each leaf
  emits the CANONICAL IMASM token code, so the trie is a genuine 16-entry decode
  table realized entirely as control flow.
- The operand two cells are passed through untouched.
- 16 opcodes × 16 operands is the whole 256-byte space, decoded with zero bytes
  of data storage. The table is topology. Verified byte for byte against the same
  table it realizes.

## The ramp to the Replicating Code

1. DONE: a decode step that computes, table-as-trie, running on parasm.
2. DONE: a full byte decoder (opcode field dispatched to the canonical token,
   operand passed through), the whole 256-byte space as pure structure.
3. DONE, three ways:
   - `test_imasm_instruction_stream_decoder`: the byte decoder wrapped in a ROTAT
     loop, streaming an instruction sequence to a full IMASM word, halting on a
     reserved stop-opcode.
   - `test_evm_lift_reentrancy_verdict`: real EVM control lifted to IMASM and
     verdicted. A withdraw whose branch paths MERGE (JUMPDEST = ∋) before the
     state commit (SSTORE = ⊡) closes (T); the vulnerable ordering, committing
     while the fork is still open, opens (B). Reentrancy caught structurally, no
     Solidity knowledge in the engine.
   - `ob3ect/test_cpython_lift.py`: the same on real CPython bytecode through the
     Python SIXTEEN_3 engine. A guarded update that merges before commit closes
     (T); one that commits inside a branch and returns early, so the paths never
     rejoin, opens (B).
   One law across two ISAs: a fork that commits before its paths rejoin is a
   leak, and the grammar names it. This is the bughunter thesis at the bytecode
   level, running.
4. DONE: `test_imasm_recompile_is_inverse`. The disassembler D lifts a wire
   opcode to the canonical one (D = the scramble); the recompiler R fuses it
   back (R = its inverse). Both are IMASM-native tries generated independently
   from inverse tables, and R(D(code)) recovers the byte for every opcode. The
   recompiler is a true inverse, μ∘δ = id, the b4_diff_scanner pattern promoted
   to a compiler.
5. DONE: `test_imasm_replicating_fixed_point`. Because R∘D is identity on the
   whole opcode space, it is identity on the tool's own word. The tool is written
   in the twelve, so the twelve fed through disassemble-then-recompile come back
   unchanged: the tool reproduces its own alphabet.
6. DONE: `test_imasm_self_hosting_quine`. The tool's own word ⊢∈>⊤<⊥∋⊡⊣ (open the
   fork, work both arms, fuse, commit, close) is verdicted T by the kernel — the
   tool is a well-formed CLOSING grammar object — and each of its tokens runs
   through the tool (disassemble then recompile) back to itself. This closes the
   Replicating Code, and there is nothing further "outside" it: the tool's word,
   its byte encoding, and its self-application co-type. They are one object, which
   is what ⊙ (imscription, a boundary around its own centre) names. Code is data
   is word; nothing is one primitive away because nothing is outside the twelve.
   δ opens, μ closes, μ∘δ = id, and the pair is the tool reading itself.

Every plank runs under `cargo test --features hosted` (16 parasm tests) plus the
CPython/EVM companions. The disassembler is written in IMASM, executed by parasm
over the crystal, decodes real EVM and CPython bytecode to IMASM words the kernel
verdicts, recompiles them back exactly, and reproduces its own closing word.
Everything is within the Grammar. No non-IMASM part, and no outside.

## The EVM Lane: bytecode-to-grammar, written in the twelve

### `test_evm_lane_in_parasm` — the lifter as a parasm word

The Replicating Code's plank 3 lifted EVM control flow *about* the grammar
(`test_evm_lift_reentrancy_verdict` builds the IMASM word in Rust, then feeds it
to parasm to verdict it). Plank 3b inverts the direction: the **lifter itself**
is written as an IMASM program and runs *inside* parasm.

The parasm program `test_evm_lane_in_parasm` does the following:

- Reads EVM opcode bytes from the kernel's input buffer, four B4 cells per byte.
- Dispatches each byte through a control-flow trie: `JT`/`JF`/`JB`/`JN` per
  cell, nesting four deep to form the full 256-way byte decode. This is the
  same dispatch-trie technique as the byte decoder above — but now the trie
  is the *lifter*, not a test scaffold.
- Maps each opcode to its IMASM token:
  
  | EVM opcode | B4 cells | IMASM glyph  |
  |------------|----------|-------------|
  | STOP (0x00)| 0,0,0,0  | ⊢ (VINIT)    |
  | SLOAD (0x54)| 1,1,1,0 | > (AFWD)     |
  | SSTORE (0x55)| 1,1,1,1| ⊡ (IFIX)     |
  | JUMPI (0x57)| 1,1,1,3| ⋈ (CLINK)    |
  | JUMPDEST (0x5b)| 1,1,2,3| ∋ (FFUSE)  |
  | PUSH1 (0x60)| 1,2,0,0 | skip operand | 
  | CALL (0xf1)| 3,3,0,1  | > (AFWD)     |
  | RETURN (0xf3)| 3,3,0,3| ⊢ (TANCH)    |
  | REVERT (0xfd)| 3,3,3,1| ⊢ (TANCH)    |
  | 0xfe sentinel| 3,3,3,2| HALT         |

- Skips PUSH1 operands (READ %r5 × 4 = skip 4 cells = 1 byte).
- Emits the lifted IMASM word through the emit buffer.
- The emitted word is then verdicted by `imasm16_3::tri_ancestral_verdict`,
  also the grammar's own parser — no Rust in the verdict path either.

**What this rung proves:** the lifter is itself a parasm program (a sequence of
B4 cells), it runs entirely within the grammar, and it produces the same IMASM
word the reference lifters emit — from real EVM bytes, with no Rust or Python
anywhere in the lift path. The bytes enter through the kernel's input, the trie
dispatches, and the word comes out.

### The Exotic One-Shots

**File:** `src/exotic_one_shots.rs` (612 lines)  
**Concept:** ig-docs/exotic_1.md — ten non-obvious fixed-point nestings

The Fixed-Point Nesting Rule: *A nesting of A inside B closes exactly when A is
a fixed point of B's action, and it closes in one shot exactly when A already
sits at that fixed point.* The exotic one-shots are non-obvious constructions
where the inner object is already the outer action's fixed point — not through
trivial identity, but through deep structural coincidences across documented
domains.

| # | Name | Domain | Outer B | Inner A | One-Shot |
|---|------|--------|---------|---------|----------|
| 1 | Winding Preimage | Number theory | Winding = 2π | period r = ord_N(a) | r is the winding-zero; BSGS measures radius, doesn't iterate |
| 2 | Belnap B-Fixed | Logic/QC | Hadamard H | dialetheic B | H\|B⟩ = \|B⟩; ¬B = B; B is native superposition state |
| 3 | O∞ Crystal Tier | Category theory | Tier O₂† → O∞ | the grammar itself | Grammar already sits at O∞; last door is structural |
| 4 | Reconnection X-Point | Plasma physics | Reconnection operator | the X-point itself | X-point is the defect lines reconnect *around* |
| 5 | Grammar Σ=1:1 Limit | SIC-POVM | Multilattice closure | universal grammar | Grammar differs from multilattice SIC only in Σ |
| 6 | Type Convergence | Category theory | OVM type convergence | Grammar ⋈ fidelity | Live ⋈ check: grammar's fidelity is quantum (peep) |
| 7 | Phases Off Lattice | Fibonacci QC | Generator lattice (tenths) | Jones phase at 1/8 | T = 1/8 ∉ 1/10 lattice → no braid reaches it exactly |
| 8 | Solovay-Kitaev Floor | Gate compilation | Full recursive SK | Off-lattice phase | The floor follows from #7's live measurement |
| 9 | dlog Order Oracle | Shor's algorithm | Modular exp map | Order r | r is fixed point of x↦a^x; `winding_period::factor` is the engine |
| 10 | Two-Faced Frontier | Emergence | Frontier saturation | Record pair (alkahest, pythagorean) | Live tuple_distance checked against √11≈3.3176 record |

Each one-shot calls the **kernel's own** engines — `winding_period::winding_order`,
`belnap::B4::bnot`, `catalog::lookup`, `algebra::tuple_distance`,
`fibonacci_qc::jones_polynomial`/`winding_of` — never a local reimplementation
that could drift from the source of truth.

**Runner:** `exotic_ones::run_all()` executes all ten and returns formatted
reports. `exotic_ones::report()` returns a bannered summary.

---

The disassembler is written in IMASM, executed by parasm, decodes real EVM and CPMython bytecode to IMASM words the kernel verdicts, recompiles them back exactly, reproduces its own closing word, and now the **lifter itself** is a parasm program — not Rust wrapping the grammar, but the grammar wrapping itself.