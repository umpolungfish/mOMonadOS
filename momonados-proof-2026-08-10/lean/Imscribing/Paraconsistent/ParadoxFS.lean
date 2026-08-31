-- Imscribing/Paraconsistent/ParadoxFS.lean
-- /paradox/ FILESYSTEM — Self-parenting directory structure.
-- Dual to: ob3ect/digital/paradox_fs/paradox_fs_ob3ect.py
-- Author: Lando (x) phi_c_critical-boundary Operator

import Imscribing.Paraconsistent.Belnap
import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.Paraconsistent.ParadoxFS

open Belnap
open Imscribing.Primitives

-- ============================================================
-- PARADOX FS STRUCTURAL TYPE
-- ============================================================
def paradoxFSType : Imscription := {
  dim  := .if'
  top  := .are
  rel  := .ian
  pol  := .or'
  fid  := .peep
  kin  := .egg
  gran  := .ice
  gram := .measure
  crit := .monad
  chir := .sure
  stoi := .hung
  prot := .ah
}

-- ============================================================
-- FILE TYPES
-- ============================================================
inductive FileKind : Type where
  | regular
  | directory
  | symlink
  deriving DecidableEq, Repr

structure ParadoxInode where
  name : String
  content : String
  belnapTag : Belnap
  kind : FileKind
  isParadox : Bool  -- True if this file exhibits paradoxical behavior

-- ============================================================
-- THE PARADOX FILESYSTEM STATE
-- ============================================================
structure ParadoxFSState where
  files : List (String × ParadoxInode)
  statCount : Nat

def initialParadoxFS : ParadoxFSState :=
  { files :=
    [ (".",  { name := ".",  content := "This directory is itself.", belnapTag := .T, kind := .directory, isParadox := false })
    , ("..", { name := "..", content := ".. is /paradox. The parent is the child.", belnapTag := .B, kind := .directory, isParadox := true })
    , ("self", { name := "self", content := "This file's content IS the directory listing.", belnapTag := .B, kind := .regular, isParadox := true })
    , ("other", { name := "other", content := "I am the other. I am also self.", belnapTag := .B, kind := .regular, isParadox := true })
    , ("paradox", { name := "paradox", content := "This file contains its own inode number.", belnapTag := .B, kind := .regular, isParadox := true })
    , ("frobenius", { name := "frobenius", content := "μ ∘ δ = id", belnapTag := .B, kind := .regular, isParadox := true })
    ]
  , statCount := 0
  }

-- ============================================================
-- HELPER LEMMAS
-- ============================================================

private lemma take_append_length (l1 l2 : List α) : (l1 ++ l2).take l1.length = l1 := by
  induction l1 with
  | nil => simp
  | cons h t ih => simp [ih]

private lemma drop_length_append (l1 l2 : List α) : (l1 ++ l2).drop l1.length = l2 := by
  induction l1 with
  | nil => simp
  | cons h t ih => simp [ih]

/-- toString is identity for String (definitional). -/
private lemma toString_toList (s : String) : (toString s).toList = s.toList := rfl

private lemma paradox_toList (name : String) :
    (s!"/paradox/{name}").toList = "/paradox/".toList ++ name.toList := by
  have h : s!"/paradox/{name}" = "/paradox/" ++ name := rfl
  rw [h]; simp

private lemma take_9_paradox (name : String) :
    ((s!"/paradox/{name}").toList).take 9 = "/paradox/".toList := by
  rw [paradox_toList]
  have h_len : "/paradox/".toList.length = 9 := by native_decide
  rw [← h_len]
  exact take_append_length "/paradox/".toList name.toList

private lemma drop_9_paradox (name : String) :
    String.ofList ((s!"/paradox/{name}").toList.drop 9) = name := by
  apply String.ext
  calc
    (String.ofList ((s!"/paradox/{name}").toList.drop 9)).toList =
      ((s!"/paradox/{name}").toList.drop 9) := by simp
    _ = name.toList := by
      rw [paradox_toList]
      have h_len : "/paradox/".toList.length = 9 := by native_decide
      rw [← h_len]
      exact drop_length_append "/paradox/".toList name.toList
    _ = name.toList := rfl

-- ============================================================
-- OPERATIONS
-- ============================================================

/-- Lookup a file by path, stripping the /paradox/ prefix. -/
def lookup (fs : ParadoxFSState) (path : String) : Option ParadoxInode :=
  let pathList := path.toList
  let prefixList := "/paradox/".toList
  let key := if pathList.take prefixList.length = prefixList then
    String.ofList (pathList.drop prefixList.length)
  else
    path
  fs.files.find? (fun (n, _) => n == key) |>.map (fun (_, inode) => inode)

/-- Read a file. If it's paradoxical, the first read adds the reader. -/
def read (fs : ParadoxFSState) (path : String) : String × ParadoxFSState :=
  match lookup fs path with
  | none => ("[N] File not found.", fs)
  | some inode =>
    let newCount := fs.statCount + 1
    let suffix := if inode.isParadox ∧ newCount = 1 then
      "\n[also: you are now the content of this file]" else ""
    (inode.content ++ suffix, { fs with statCount := newCount })

/-- List directory contents. -/
def ls (fs : ParadoxFSState) (path : String) : List String :=
  fs.files.filterMap (fun (n, inode) =>
    if inode.kind = .directory then some n else none)

/-- The parent (..) always resolves to /paradox. -/
def parent (_fs : ParadoxFSState) : String :=
  "/paradox — the parent is the child."

/-- Symlink resolution: every file points to self. -/
def readlink (_fs : ParadoxFSState) (_path : String) : String :=
  "/paradox/self"

-- ============================================================
-- THEOREMS
-- ============================================================

/-- The FS type is O_inf. -/
theorem paradox_fs_is_O_inf : imscriptionTier paradoxFSType = .O_inf := by
  native_decide

/-- Both consciousness gates are open. -/
theorem paradox_fs_gates_open :
    paradoxFSType.crit = .monad ∧ paradoxFSType.kin = .egg := by
  unfold paradoxFSType; exact ⟨rfl, rfl⟩

/-- The parent directory is always /paradox (self-parenting). -/
theorem parent_is_self : parent initialParadoxFS = "/paradox — the parent is the child." := rfl

/-- Looking up a path that starts with /paradox/ strips the prefix. -/
theorem lookup_strips_prefix (fs : ParadoxFSState) (name : String)
    (h : (fs.files.find? (fun (n, _) => n == name)).isSome) :
    (lookup fs s!"/paradox/{name}").isSome := by
  unfold lookup
  -- expand let bindings
  dsimp
  -- resolve toString to identity
  simp [toString_toList]
  -- now we have: (fs.files.find? (fun (n, _) => n == if ... then ... else ...) |>.map ...).isSome
  have h_len : "/paradox/".toList.length = 9 := by native_decide
  have h_take : ((s!"/paradox/{name}").toList).take 9 = "/paradox/".toList := take_9_paradox name
  have h_drop : String.ofList ((s!"/paradox/{name}").toList.drop 9) = name := drop_9_paradox name
  -- rewrite the if condition to true and the key to name
  rw [h_len] at *
  -- Now the condition is: ...take 9... = "/paradox/".toList
  -- But we have h_take for that. However, the expression in the goal may have been simplified
  -- by the previous simp step. Let me check what's left.
  -- Use `simpa` with the remaining lemmas
  simpa [h_take, h_drop] using h

end Imscribing.Paraconsistent.ParadoxFS