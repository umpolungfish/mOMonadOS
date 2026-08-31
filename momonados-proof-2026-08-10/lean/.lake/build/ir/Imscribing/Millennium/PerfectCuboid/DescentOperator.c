// Lean compiler output
// Module: Imscribing.Millennium.PerfectCuboid.DescentOperator
// Imports: public import Init public import Imscribing.Millennium.PerfectCuboid public import Imscribing.Millennium.PerfectCuboid.DescentTheorem public import Mathlib.Data.Nat.Basic public import Mathlib.Tactic
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
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core___boxed(lean_object**);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core___redArg(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7) {
_start:
{
lean_object* x_8; 
x_8 = lean_alloc_ctor(0, 7, 0);
lean_ctor_set(x_8, 0, x_1);
lean_ctor_set(x_8, 1, x_2);
lean_ctor_set(x_8, 2, x_3);
lean_ctor_set(x_8, 3, x_4);
lean_ctor_set(x_8, 4, x_5);
lean_ctor_set(x_8, 5, x_6);
lean_ctor_set(x_8, 6, x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5, lean_object* x_6, lean_object* x_7, lean_object* x_8, lean_object* x_9, lean_object* x_10, lean_object* x_11, lean_object* x_12, lean_object* x_13, lean_object* x_14, lean_object* x_15, lean_object* x_16, lean_object* x_17, lean_object* x_18, lean_object* x_19) {
_start:
{
lean_object* x_20; 
x_20 = lean_alloc_ctor(0, 7, 0);
lean_ctor_set(x_20, 0, x_2);
lean_ctor_set(x_20, 1, x_3);
lean_ctor_set(x_20, 2, x_4);
lean_ctor_set(x_20, 3, x_5);
lean_ctor_set(x_20, 4, x_6);
lean_ctor_set(x_20, 5, x_7);
lean_ctor_set(x_20, 6, x_8);
return x_20;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core___boxed(lean_object** _args) {
lean_object* x_1 = _args[0];
lean_object* x_2 = _args[1];
lean_object* x_3 = _args[2];
lean_object* x_4 = _args[3];
lean_object* x_5 = _args[4];
lean_object* x_6 = _args[5];
lean_object* x_7 = _args[6];
lean_object* x_8 = _args[7];
lean_object* x_9 = _args[8];
lean_object* x_10 = _args[9];
lean_object* x_11 = _args[10];
lean_object* x_12 = _args[11];
lean_object* x_13 = _args[12];
lean_object* x_14 = _args[13];
lean_object* x_15 = _args[14];
lean_object* x_16 = _args[15];
lean_object* x_17 = _args[16];
lean_object* x_18 = _args[17];
lean_object* x_19 = _args[18];
_start:
{
lean_object* x_20; 
x_20 = lp_imscribing_x2dlean_Millennium_PerfectCuboid_halve__cuboid__core(x_1, x_2, x_3, x_4, x_5, x_6, x_7, x_8, x_9, x_10, x_11, x_12, x_13, x_14, x_15, x_16, x_17, x_18, x_19);
lean_dec_ref(x_1);
return x_20;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid_DescentTheorem(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Nat_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Tactic(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid_DescentOperator(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Millennium_PerfectCuboid_DescentTheorem(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Nat_Basic(builtin);
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
