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
4. The inverse: recompile an IMASM word plus its payload back to the target,
   diff against the original (the b4_diff_scanner pattern promoted to a
   compiler). μ∘δ = id where the round trip closes.
5. Turn the tool on its own parasm word. When it disassembles to itself and
   recompiles to itself, the fixed point exists and universality is settled by
   construction. That is the Replicating Code, and the demonstration is the
   quine, not an argument about it.
