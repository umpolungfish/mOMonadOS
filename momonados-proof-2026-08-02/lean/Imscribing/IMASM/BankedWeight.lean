/-
# Banked weight: what survives a clear

The IMASM trilattice machine holds a register and a stack of open fork frames.
A deposit writes to both. A reversal (AREV) empties the REGISTER and leaves the
frames untouched, so weight banked in a frame survives a clear that destroys
everything held in the open. A fuse pops a frame and folds it back.

Two facts about this are independent and were repeatedly conflated:

1. **Whether anything is banked at all** depends on whether a frame is still
   open when the clear fires. That is governed by how many fuses have already
   run, not by where the splits sit relative to the deposits.

2. **Whether a surplus is kept** depends on where the splits fall between
   deposits of the same value. Deposits inside one region accumulate; the fold
   between regions takes a maximum, so a value deposited in two sibling regions
   arrives as one.

Both are proved below. The set-valued machine is the case where every weight is
0 or 1, and there `max` is union and the whole account degenerates to
membership, which is why the base machine cannot see any of this.
-/

import Mathlib.Data.Nat.Basic
import Mathlib.Order.Basic
import Mathlib.Tactic

namespace Imscribing.IMASM

variable {V : Type*} [DecidableEq V]

/-- A weight assignment: how many times each base value has been deposited. -/
abbrev Weights (V : Type*) := V → ℕ

/-- Machine state: the register held in the open, and the stack of open frames.
    `frames.head` is the innermost. -/
structure State (V : Type*) where
  reg : Weights V
  frames : List (Weights V)

namespace State

/-- Deposit one unit of `v`. It lands in the register AND in the innermost open
    frame, in one action: the doubling is not created by the nesting, only
    placed by it. -/
def touch (s : State V) (v : V) : State V :=
  { reg := fun w => if w = v then s.reg w + 1 else s.reg w,
    frames := match s.frames with
      | [] => []
      | f :: rest => (fun w => if w = v then f w + 1 else f w) :: rest }

/-- Open a frame. -/
def split (s : State V) : State V :=
  { s with frames := (fun _ => 0) :: s.frames }

/-- Close the innermost frame, folding it outward by MAX.

    Max rather than sum: the base machine fuses by union, which is idempotent,
    so a value already present is unaffected by being re-emitted. Summing would
    count each deposit twice, once landing in the register and again when its
    frame closed. -/
def fuse (s : State V) : State V :=
  match s.frames with
  | [] => s
  | f :: rest =>
      { reg := fun w => max (s.reg w) (f w),
        frames := match rest with
          | [] => []
          | g :: tl => (fun w => max (g w) (f w)) :: tl }

/-- A reversal. Empties the register; open frames are untouched. -/
def clear (s : State V) : State V :=
  { s with reg := fun _ => 0 }

end State

open State

/-! ## Fact 1 — banking depends on a frame being open at the clear -/

/-- With a frame open, a clear followed by a fuse restores exactly what that
    frame held. This is the technique: the count survives the reversal. -/
theorem restored_of_frame_open (s : State V) (f : Weights V) (rest : List (Weights V))
    (h : s.frames = f :: rest) (v : V) :
    (s.clear.fuse).reg v = f v := by
  simp [State.clear, State.fuse, h]

/-- With no frame open, a clear takes everything and a following fuse restores
    nothing: the fuse has no frame to pop. This is the failure the diagnostic
    reports as `banked: 0`. -/
theorem lost_of_no_frame (s : State V) (h : s.frames = []) (v : V) :
    (s.clear.fuse).reg v = 0 := by
  simp [State.clear, State.fuse, h]

/-- The two are exhaustive: after a clear, what a fuse can give back is entirely
    determined by whether the stack is empty. Nothing about the deposits, their
    count, or the placement of earlier splits enters. -/
theorem banking_dichotomy (s : State V) (v : V) :
    (s.clear.fuse).reg v = (s.frames.head?.map (fun f => f v)).getD 0 := by
  cases s with
  | mk reg frames =>
    cases frames with
    | nil => simp [State.clear, State.fuse]
    | cons f rest => simp [State.clear, State.fuse]

/-! ## Fact 2 — the surplus depends on where the splits fall -/

/-- Two deposits of the same value inside ONE region accumulate: the frame
    carries both. -/
theorem same_region_accumulates (s : State V) (f : Weights V) (rest : List (Weights V))
    (h : s.frames = f :: rest) (v : V) :
    ((s.touch v).touch v).frames.head?.map (fun g => g v) = some (f v + 2) := by
  simp [State.touch, h]

/-- The REGISTER keeps both deposits however they are arranged: `touch`
    increments it every time, and no fold ever reduces it. The flattening
    happens in the frames, not in the open. -/
theorem register_keeps_both (s : State V) (v : V) (h : s.frames = []) :
    ((((s.split.touch v).split.touch v).fuse).fuse).reg v = s.reg v + 2 := by
  simp [State.split, State.touch, State.fuse, h]

/-- The contrast, stated where it is observable: what a clear leaves behind.

    Two deposits in ONE region are banked as two and come back as two. The same
    two deposits split across SIBLING regions are folded by max to one, and one
    is what returns. Same deposit count, same number of fuses before the clear,
    different surplus — and the only difference is the split between them. -/
theorem same_region_restores_two (s : State V) (v : V) (h : s.frames = []) :
    ((((s.split.touch v).touch v).clear).fuse).reg v = 2 := by
  simp [State.split, State.touch, State.clear, State.fuse, h]

theorem sibling_regions_restore_one (s : State V) (v : V) (h : s.frames = []) :
    (((((s.split.touch v).split.touch v).fuse).clear).fuse).reg v = 1 := by
  simp [State.split, State.touch, State.clear, State.fuse, h]

/-- A deposit writes to the register and to the innermost frame in the same
    action, so the second copy exists before any fuse runs. The nesting decides
    WHERE that copy sits, not whether there is one. -/
theorem touch_writes_both (s : State V) (f : Weights V) (rest : List (Weights V))
    (h : s.frames = f :: rest) (v : V) :
    (s.touch v).reg v = s.reg v + 1 ∧
    (s.touch v).frames.head?.map (fun g => g v) = some (f v + 1) := by
  constructor <;> simp [State.touch, h]

/-! ## The two facts are independent -/

/-- A word can bank correctly and still discard a surplus, and a word can keep
    every unit and bank nothing. Neither fact implies the other, which is why a
    rule stated in terms of split placement alone cannot decide whether anything
    survives, and a rule stated in terms of fuse count alone cannot decide how
    much. -/
theorem independence (s : State V) (v : V) (h : s.frames = []) :
    -- a frame IS open at the clear, so something is banked, yet the sibling
    -- fold has already flattened two deposits to one
    (((((s.split.touch v).split.touch v).fuse).clear).fuse).reg v = 1 ∧
    -- both deposits in one region, but the fuse ran BEFORE the clear, so no
    -- frame remains and everything goes however carefully it was deposited
    (((((s.split.touch v).touch v).fuse).clear).fuse).reg v = 0 := by
  refine ⟨?_, ?_⟩ <;> simp [State.split, State.touch, State.fuse, State.clear, h]

end Imscribing.IMASM
