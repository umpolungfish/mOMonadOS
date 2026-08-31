// Lean compiler output
// Module: Imscribing.Primitives.TierCrossing
// Imports: public import Init public import Mathlib.Analysis.SpecialFunctions.Log.Basic public import Mathlib.Analysis.SpecialFunctions.Pow.Real public import Imscribing.Primitives.Imscription
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
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularitySeparation(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg(uint8_t, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_to_int(lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularitySeparation___boxed(lean_object*, lean_object*);
lean_object* lean_int_sub(lean_object*, lean_object*);
lean_object* lean_nat_abs(lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel(uint8_t);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter(lean_object*, uint8_t, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel(uint8_t x_1) {
_start:
{
switch (x_1) {
case 0:
{
lean_object* x_2; 
x_2 = lean_unsigned_to_nat(0u);
return x_2;
}
case 1:
{
lean_object* x_3; 
x_3 = lean_unsigned_to_nat(1u);
return x_3;
}
default: 
{
lean_object* x_4; 
x_4 = lean_unsigned_to_nat(2u);
return x_4;
}
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lean_unbox(x_1);
x_3 = lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularitySeparation(uint8_t x_1, uint8_t x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; 
x_3 = lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel(x_2);
x_4 = lean_nat_to_int(x_3);
x_5 = lp_imscribing_x2dlean_Imscribing_TierCrossing_granularityLevel(x_1);
x_6 = lean_nat_to_int(x_5);
x_7 = lean_int_sub(x_4, x_6);
lean_dec(x_6);
lean_dec(x_4);
x_8 = lean_nat_abs(x_7);
lean_dec(x_7);
return x_8;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_TierCrossing_granularitySeparation___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; uint8_t x_4; lean_object* x_5; 
x_3 = lean_unbox(x_1);
x_4 = lean_unbox(x_2);
x_5 = lp_imscribing_x2dlean_Imscribing_TierCrossing_granularitySeparation(x_3, x_4);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg(uint8_t x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
switch (x_1) {
case 0:
{
lean_object* x_5; lean_object* x_6; 
lean_dec(x_4);
lean_dec(x_3);
x_5 = lean_box(0);
x_6 = lean_apply_1(x_2, x_5);
return x_6;
}
case 1:
{
lean_object* x_7; lean_object* x_8; 
lean_dec(x_4);
lean_dec(x_2);
x_7 = lean_box(0);
x_8 = lean_apply_1(x_3, x_7);
return x_8;
}
default: 
{
lean_object* x_9; lean_object* x_10; 
lean_dec(x_3);
lean_dec(x_2);
x_9 = lean_box(0);
x_10 = lean_apply_1(x_4, x_9);
return x_10;
}
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_1);
x_6 = lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg(x_5, x_2, x_3, x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_object* x_6; 
x_6 = lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___redArg(x_2, x_3, x_4, x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; lean_object* x_7; 
x_6 = lean_unbox(x_2);
x_7 = lp_imscribing_x2dlean___private_Imscribing_Primitives_TierCrossing_0__Imscribing_TierCrossing_granularityLevel_match__1_splitter(x_1, x_6, x_3, x_4, x_5);
return x_7;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Analysis_SpecialFunctions_Log_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Analysis_SpecialFunctions_Pow_Real(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Primitives_Imscription(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_imscribing_x2dlean_Imscribing_Primitives_TierCrossing(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Analysis_SpecialFunctions_Log_Basic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Analysis_SpecialFunctions_Pow_Real(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Primitives_Imscription(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
