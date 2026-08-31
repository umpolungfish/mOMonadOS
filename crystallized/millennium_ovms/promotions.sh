#!/usr/bin/env bash
# Millennium problems as operator-valued measures — promotion reproducer.
#
# TWO LEVELS, deliberately kept apart:
#   kernel   `imasm derive <word>` RUNS the word as a program and reports the
#            structural witness of its control flow.
#   design   the ob3ect's grounded tuple READS the description against the
#            twelve axes. A different measurement of a different thing.
# Their addresses are NOT comparable. What is comparable is the parity slot ≺.
# Exit 0 = PASS.
set -uo pipefail
cd "$(dirname "$0")/../.."
RUN=./run_hosted_cmds.sh

declare -A WORD=(
 [rh_positivity]="⊢∈≻⊤≺⊥⊞⋈∋⊡⊙⊣"
 [bsd_parity]="⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣"
 [hodge_parity]="⊢∈⊥≺⊤≻⋈⊞∋⊙⊡⊣"
 [collatz_or_closure]="⊢∈≻⊤≺⊥⋈⊞⊙∋⊡⋈⊙⊣"
 [collatz_named_move]="⊢∈⊤≻⋈⊥≺⋈⊞⊡⊙∋⋈∈⊤≻⊥≺⊞⊡∋⊣"
)
# kernel crystal, kernel parity ≺, design parity ≺ (from the ob3ect grounding)
declare -A EXPECT=(
 [rh_positivity]="3444190 𐑹 𐑹"       [bsd_parity]="3444190 𐑹 𐑹"
 [hodge_parity]="3444190 𐑹 𐑹"        [collatz_or_closure]="3444190 𐑹 𐑹"
 [collatz_named_move]="16404190 𐑹 𐑿"
)
ORDER="rh_positivity bsd_parity hodge_parity collatz_or_closure collatz_named_move"

fail=0
printf '%-20s %-10s %-10s %-8s %-8s %s\n' NAME EXPECT GOT 'Φ-kern' 'Φ-desgn' RESULT
for k in $ORDER; do
  out=$($RUN "imasm derive ${WORD[$k]}" 2>/dev/null)
  got=$(printf '%s' "$out" | grep -oP 'crystal:\s*\K[0-9]+' | head -1)
  phi=$(printf '%s' "$out" | grep -oP 'tuple:\s*⟨\K[^⟩]+' | head -1 \
        | tr -d ' ' | awk -F'·' '{print $4}')
  read -r e_a e_p e_d <<< "${EXPECT[$k]}"
  res=PASS
  [ "$got" = "$e_a" ] && [ "$phi" = "$e_p" ] || { res=FAIL; fail=1; }
  printf '%-20s %-10s %-10s %-8s %-8s %s\n' "$k" "$e_a" "$got" "$phi" "$e_d" "$res"
done

echo
echo "--- every promotion word closes the parity to or' at kernel level ---"
echo "--- descent 3444190 and fork 16404190 differ only at ⊢ ---"
$RUN "crystal 3444190" "crystal 16404190" 2>/dev/null | grep -E '⊢:' | sed 's/^ */  /'
echo
echo "--- the self-encode type carries parity or' AND winding ℤ, not Z2 ---"
echo "--- (16840174 is its address in the KERNEL's numbering; 6734591 is the"
echo "---  same type in the boundary-cell numbering. Never mix the two.) ---"
$RUN "crystal 16840174" 2>/dev/null | grep -E '≺:|⊡:' | sed 's/^ */  ordinal /'

[ $fail -eq 0 ] && echo && echo "PROMOTIONS: PASS" || { echo; echo "PROMOTIONS: FAIL"; }
exit $fail
