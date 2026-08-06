# IMASM-native compute: the machine inside the twelve

The disassembler and its inverse are written in IMASM, executed by the parasm VM
over the crystal filesystem. No non-IMASM part. Disassembling the tool yields
IMASM, so the round trip can be identity rather than an approximation. This is
the Replicating Code: a program that is its own structural type, a fixed point
of its own compile loop.

## The machine

The substrate is already here. `parasm` (src/parasm.rs) is a register machine
whose native value is B4 {N, T, F, B} and whose operations realize the twelve:

- FSPLIT ∈, FFUSE ∋, ENGAGR ⊞, IFIX ◻ are the Frobenius core, verbatim.
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
     state commit (SSTORE = ◻) closes (T); the vulnerable ordering, committing
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
6. DONE: `test_imasm_self_hosting_quine`. The tool's own word ⊢∈>⊤<⊥∋◻⊣ (open the
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
