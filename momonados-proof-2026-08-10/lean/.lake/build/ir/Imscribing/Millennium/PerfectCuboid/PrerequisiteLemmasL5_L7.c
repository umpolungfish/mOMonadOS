// Lean compiler output
// Module: Imscribing.Millennium.PerfectCuboid.PrerequisiteLemmasL5_L7
// Imports: public import Init public import Mathlib.Data.Nat.Basic public import Mathlib.Data.Nat.GCD.Basic public import Mathlib.Data.Int.Basic public import Mathlib.Data.Int.GCD public import Mathlib.NumberTheory.PythagoreanTriples public import Mathlib.Tactic
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Nat_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Nat_GCD_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Int_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Int_GCD(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_NumberTheory_PythagoreanTriples(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Tactic(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid_PrerequisiteLemmasL5__L7(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Nat_Basic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Nat_GCD_Basic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Int_Basic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Int_GCD(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_NumberTheory_PythagoreanTriples(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Tactic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
