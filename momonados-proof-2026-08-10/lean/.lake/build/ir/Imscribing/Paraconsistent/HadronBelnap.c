// Lean compiler output
// Module: Imscribing.Paraconsistent.HadronBelnap
// Imports: public import Init public import Imscribing.Paraconsistent.QuarkBelnap public import Imscribing.Paraconsistent.OrbitalBelnap public import Imscribing.Paraconsistent.Belnap public import Imscribing.Primitives.Imscription public import Imscribing.Primitives.TierCrossing public import Mathlib.Data.Finset.Basic public import Mathlib.Tactic
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
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1;
uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_colorJoin(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeBaryon(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeMeson(lean_object*, lean_object*);
uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(uint8_t, uint8_t);
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonPair(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonDepair(lean_object*);
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon___boxed(lean_object*, lean_object*);
static lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0;
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq___boxed(lean_object*, lean_object*);
uint8_t lp_imscribing_x2dlean_Imscribing_Primitives_instDecidableEqOuroboricityTier(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonDepair___boxed(lean_object*);
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq(lean_object*, lean_object*);
uint8_t lp_imscribing_x2dlean_Imscribing_Primitives_imscriptionTier(lean_object*);
static uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1___closed__0;
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_mesonPair(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_mesonDepair(lean_object*);
uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription;
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; uint8_t x_7; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
x_5 = lean_ctor_get(x_2, 0);
x_6 = lean_ctor_get(x_2, 1);
x_7 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(x_3, x_5);
if (x_7 == 0)
{
return x_7;
}
else
{
uint8_t x_8; 
x_8 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(x_4, x_6);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq(x_1, x_2);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson_decEq(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqMeson(x_1, x_2);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeMeson(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; uint8_t x_4; uint8_t x_5; 
x_3 = lean_ctor_get_uint8(x_1, 0);
x_4 = lean_ctor_get_uint8(x_2, 0);
x_5 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_3, x_4);
if (x_5 == 0)
{
lean_object* x_6; 
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_6 = lean_box(0);
return x_6;
}
else
{
lean_object* x_7; lean_object* x_8; 
x_7 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_7, 0, x_1);
lean_ctor_set(x_7, 1, x_2);
x_8 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_8, 0, x_7);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_mesonDepair(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = !lean_is_exclusive(x_1);
if (x_2 == 0)
{
return x_1;
}
else
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
lean_inc(x_4);
lean_inc(x_3);
lean_dec(x_1);
x_5 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
return x_5;
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_mesonPair(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeMeson(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; uint8_t x_9; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
x_5 = lean_ctor_get(x_1, 2);
x_6 = lean_ctor_get(x_2, 0);
x_7 = lean_ctor_get(x_2, 1);
x_8 = lean_ctor_get(x_2, 2);
x_9 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(x_3, x_6);
if (x_9 == 0)
{
return x_9;
}
else
{
uint8_t x_10; 
x_10 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(x_4, x_7);
if (x_10 == 0)
{
return x_10;
}
else
{
uint8_t x_11; 
x_11 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqQuarkState_decEq(x_5, x_8);
return x_11;
}
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq(x_1, x_2);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT uint8_t lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon_decEq(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_instDecidableEqBaryon(x_1, x_2);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeBaryon(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_4; uint8_t x_5; uint8_t x_6; 
x_4 = lean_ctor_get_uint8(x_1, 0);
x_5 = 0;
x_6 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_4, x_5);
if (x_6 == 0)
{
uint8_t x_7; uint8_t x_8; 
x_7 = lean_ctor_get_uint8(x_2, 0);
x_8 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_7, x_5);
if (x_8 == 0)
{
uint8_t x_9; uint8_t x_10; 
x_9 = lean_ctor_get_uint8(x_3, 0);
x_10 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_9, x_5);
if (x_10 == 0)
{
uint8_t x_11; uint8_t x_12; 
x_11 = 4;
x_12 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_4, x_11);
if (x_12 == 0)
{
uint8_t x_13; 
x_13 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_7, x_11);
if (x_13 == 0)
{
uint8_t x_14; 
x_14 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_9, x_11);
if (x_14 == 0)
{
uint8_t x_15; 
x_15 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_4, x_7);
if (x_15 == 0)
{
uint8_t x_16; 
x_16 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_4, x_9);
if (x_16 == 0)
{
uint8_t x_17; 
x_17 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_7, x_9);
if (x_17 == 0)
{
uint8_t x_18; uint8_t x_19; uint8_t x_20; 
x_18 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_colorJoin(x_4, x_7);
x_19 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_colorJoin(x_18, x_9);
x_20 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap_instDecidableEqColorState(x_19, x_11);
if (x_20 == 0)
{
lean_object* x_21; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_21 = lean_box(0);
return x_21;
}
else
{
lean_object* x_22; lean_object* x_23; 
x_22 = lean_alloc_ctor(0, 3, 0);
lean_ctor_set(x_22, 0, x_1);
lean_ctor_set(x_22, 1, x_2);
lean_ctor_set(x_22, 2, x_3);
x_23 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_23, 0, x_22);
return x_23;
}
}
else
{
lean_object* x_24; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_24 = lean_box(0);
return x_24;
}
}
else
{
lean_object* x_25; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_25 = lean_box(0);
return x_25;
}
}
else
{
lean_object* x_26; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_26 = lean_box(0);
return x_26;
}
}
else
{
lean_object* x_27; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_27 = lean_box(0);
return x_27;
}
}
else
{
lean_object* x_28; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_28 = lean_box(0);
return x_28;
}
}
else
{
lean_object* x_29; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_29 = lean_box(0);
return x_29;
}
}
else
{
lean_object* x_30; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_30 = lean_box(0);
return x_30;
}
}
else
{
lean_object* x_31; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_31 = lean_box(0);
return x_31;
}
}
else
{
lean_object* x_32; 
lean_dec_ref(x_3);
lean_dec_ref(x_2);
lean_dec_ref(x_1);
x_32 = lean_box(0);
return x_32;
}
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonDepair(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_2 = lean_ctor_get(x_1, 0);
x_3 = lean_ctor_get(x_1, 1);
x_4 = lean_ctor_get(x_1, 2);
lean_inc_ref(x_4);
lean_inc_ref(x_3);
x_5 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
lean_inc_ref(x_2);
x_6 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_6, 0, x_2);
lean_ctor_set(x_6, 1, x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonDepair___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonDepair(x_1);
lean_dec_ref(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_baryonPair(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_tryMakeBaryon(x_1, x_2, x_3);
return x_4;
}
}
static lean_object* _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0() {
_start:
{
uint8_t x_1; uint8_t x_2; uint8_t x_3; uint8_t x_4; uint8_t x_5; uint8_t x_6; uint8_t x_7; uint8_t x_8; uint8_t x_9; uint8_t x_10; uint8_t x_11; uint8_t x_12; lean_object* x_13; 
x_1 = 2;
x_2 = 2;
x_3 = 2;
x_4 = 1;
x_5 = 0;
x_6 = 2;
x_7 = 2;
x_8 = 2;
x_9 = 2;
x_10 = 2;
x_11 = 2;
x_12 = 2;
x_13 = lean_alloc_ctor(0, 0, 12);
lean_ctor_set_uint8(x_13, 0, x_12);
lean_ctor_set_uint8(x_13, 1, x_11);
lean_ctor_set_uint8(x_13, 2, x_10);
lean_ctor_set_uint8(x_13, 3, x_9);
lean_ctor_set_uint8(x_13, 4, x_8);
lean_ctor_set_uint8(x_13, 5, x_7);
lean_ctor_set_uint8(x_13, 6, x_6);
lean_ctor_set_uint8(x_13, 7, x_5);
lean_ctor_set_uint8(x_13, 8, x_4);
lean_ctor_set_uint8(x_13, 9, x_3);
lean_ctor_set_uint8(x_13, 10, x_2);
lean_ctor_set_uint8(x_13, 11, x_1);
return x_13;
}
}
static lean_object* _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription() {
_start:
{
lean_object* x_1; 
x_1 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0;
return x_1;
}
}
static uint8_t _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1___closed__0() {
_start:
{
lean_object* x_1; uint8_t x_2; 
x_1 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription;
x_2 = lp_imscribing_x2dlean_Imscribing_Primitives_imscriptionTier(x_1);
return x_2;
}
}
static uint8_t _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1() {
_start:
{
uint8_t x_1; uint8_t x_2; uint8_t x_3; 
x_1 = lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1___closed__0;
x_2 = 3;
x_3 = lp_imscribing_x2dlean_Imscribing_Primitives_instDecidableEqOuroboricityTier(x_1, x_2);
return x_3;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Paraconsistent_OrbitalBelnap(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Paraconsistent_Belnap(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Primitives_Imscription(uint8_t builtin);
lean_object* initialize_imscribing_x2dlean_Imscribing_Primitives_TierCrossing(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Data_Finset_Basic(uint8_t builtin);
lean_object* initialize_mathlib_Mathlib_Tactic(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Paraconsistent_QuarkBelnap(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Paraconsistent_OrbitalBelnap(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Paraconsistent_Belnap(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Primitives_Imscription(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_imscribing_x2dlean_Imscribing_Primitives_TierCrossing(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Data_Finset_Basic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mathlib_Mathlib_Tactic(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0 = _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0();
lean_mark_persistent(lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription___closed__0);
lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription = _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription();
lean_mark_persistent(lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadronBelnapImscription);
lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1___closed__0 = _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1___closed__0();
lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1 = _init_lp_imscribing_x2dlean_Imscribing_Paraconsistent_HadronBelnap_hadron__belnap__tier___nativeDecide__1__1();
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
