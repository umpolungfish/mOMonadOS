import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.Algebra
open Imscribing.Primitives

#check compute_meet
#check scalarField_Kslow

-- Test: meet(a, a) = a
example (a : Imscription) : compute_meet a a = a := by
  unfold compute_meet
  ext <;> simp

-- Test: tensorProduct(a, a) = a
example (a : Imscription) : tensorProduct a a = a := by
  unfold tensorProduct
  ext <;> simp

-- Test: join(bottom, a) = a
def frobenius_bottom : Imscription := {
  dim := dead, top := judge, rel := ado, pol := church,
  fid := age, kin := yea, gran := bib, gram := vow,
  crit := woe, chir := fee, stoi := hung, prot := awe
}
example (a : Imscription) : compute_join frobenius_bottom a = a := by
  unfold compute_join frobenius_bottom
  ext <;> simp
