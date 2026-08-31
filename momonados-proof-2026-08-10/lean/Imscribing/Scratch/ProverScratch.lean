import Imscribing.IGFunctor
import Imscribing.Paraconsistent.BelnapSplitFuse
set_option linter.style.setOption false
set_option linter.style.whitespace false
set_option linter.style.commandStart false
set_option linter.style.longLine false
set_option linter.style.multiGoal false
set_option linter.style.cdot false
set_option linter.style.emptyLine false
set_option linter.style.dollarSyntax false
set_option linter.style.lambdaSyntax false
set_option linter.style.refine false
set_option linter.style.show false
set_option linter.style.cases false
set_option linter.style.induction false
set_option linter.style.nameCheck false
set_option linter.style.docString false
set_option linter.style.header false
set_option linter.style.missingEnd false
set_option linter.style.openClassical false
set_option linter.style.nativeDecide false
set_option linter.style.admit false
open Belnap
-- the closure carries a transformation: B splits to DISTINCT arms (T,F)
-- and fuses back (μ∘δ=id). Not a diagonal identity copy.
theorem imasm_genuine_closure :
    (fsplit Belnap.B).1 ≠ (fsplit Belnap.B).2 ∧ ffuse (fsplit Belnap.B) = Belnap.B := by
  exact ⟨by decide, split_fuse_id _⟩
