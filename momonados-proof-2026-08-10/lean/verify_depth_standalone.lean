-- Standalone verifier (Math⊙perator, 2026)
-- Re-declares the EXACT IGProtocol inductive + depth measure from
-- Imscribing/IGMorphism.lean and the EXACT `protocol` term from
-- Imscribing/Millennium/RiemannSIC.lean, then proves protocol.depth = 5.
-- depth never inspects Imscription fields, so a trivial struct suffices.
-- Grammar/Chirality are minimal local enums (only values matter, not defs).
-- Run directly with stage1 lean (no lake -> avoids broken importGraph dep).

namespace Verif

inductive Grammar : Type where
  | measure

inductive Chirality : Type where
  | wool

structure Imscription : Type where
  unit_ : Unit

def M : Imscription := { unit_ := () }

def tensorProduct (x y : Imscription) : Imscription := M

inductive IGProtocol : Imscription -> Imscription -> Type where
  | refl   : (s : Imscription) -> IGProtocol s s
  | arrow  : (label src tgt : Imscription) -> IGProtocol src tgt
  | seq    : IGProtocol a b -> IGProtocol b c -> IGProtocol a c
  | prod   : IGProtocol a b -> IGProtocol a c -> IGProtocol a (tensorProduct b c)
  | withGram : Grammar -> IGProtocol a b -> IGProtocol a b
  | withMem  : Chirality -> IGProtocol a b -> IGProtocol a b

def IGProtocol.depth : IGProtocol a b -> Nat
  | .refl _        => 0
  | .arrow _ _ _   => 1
  | .seq f g       => f.depth + g.depth
  | .prod f g      => max f.depth g.depth
  | .withGram _ p  => p.depth
  | .withMem _ p   => p.depth

def measure : Grammar := Grammar.measure
def wool    : Chirality := Chirality.wool

def l0  : Imscription := M
def l1  : Imscription := M
def l8  : Imscription := M
def l9  : Imscription := M
def l10 : Imscription := M
def s0  : Imscription := M
def s1  : Imscription := M
def s8  : Imscription := M
def s9  : Imscription := M
def s10 : Imscription := M

-- EXACT protocol term from RiemannSIC.lean (lines 228-236).
def protocol : IGProtocol s0 s10 :=
  .withGram measure <|
  (.seq (.arrow l0 s0 s1) (.seq
    (.prod (.arrow l1 s1 s8) (.arrow l1 s1 s8))
    (.seq (.arrow l8 s8 s8)
      (.seq (.arrow l8 s8 s9) (.arrow l9 s9 s10)))))

-- Corrected theorem: actual structural depth is 5, NOT 11.
theorem protocol_depth_is_5 : protocol.depth = 5 := by
  native_decide

-- Diagnostic: narrative claims 11 opcodes; constructed term has depth 5.
theorem depth_narrative_mismatch :
    protocol.depth = 5 /\ (protocol.depth != 11) := by
  native_decide

end Verif
