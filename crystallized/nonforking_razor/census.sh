#!/usr/bin/env bash
# Non-Forking Razor — census reproducer (crystallized proof for ig-docs/nonforking_razor.tex)
# Reproduces: ten razor words (five conjectures × affirm/negate) + two crystal decodes.
# Each row asserts the derived crystal; every word must bank OK, close vox T, classify ⊞.
set -uo pipefail
cd "$(dirname "$0")/../.."           # -> mOMonadOS root
RUN=./run_hosted_cmds.sh

declare -A WORD=(
 [rh_affirm]="⊢∈≻⊤≺⊥⊞⋈⊙∋⊡⊣"          [rh_negate]="⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣"
 [collatz_affirm]="⊢∈⊤≺⊥≻≺∋⋈⊞⊡⊙∈⊤≺⊥≻≺∋⋈⊡⊣" [collatz_negate]="⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣"
 [bsd_affirm]="⊢∈≻⊤≺⊥⋈⊞⊙∋⊡⊣"         [bsd_negate]="⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣"
 [hodge_affirm]="⊢∈≻⊤⋈≺⊥⊞∋⊙⊡⋈⊙⊣"     [hodge_negate]="⊢∈≻⊤⋈⊙≺⊥⊞⊡∋⋈⊙⊣"
 [ns_affirm]="⊢∈≻⊤≺⊥∋⋈⊞⊡⊙⊣"          [ns_negate]="⊢∈≻⊤≺⊥⋈⊙⊞∋⊡⊣"
)
declare -A EXPECT=(
 [rh_affirm]=3444190  [rh_negate]=3444190
 [collatz_affirm]=16404190 [collatz_negate]=3444190
 [bsd_affirm]=3444190 [bsd_negate]=3444190
 [hodge_affirm]=3444190 [hodge_negate]=3444190
 [ns_affirm]=3444190  [ns_negate]=3444190
)
ORDER="rh_affirm rh_negate collatz_affirm collatz_negate bsd_affirm bsd_negate hodge_affirm hodge_negate ns_affirm ns_negate"

fail=0
printf '%-16s %-10s %-10s %-6s %-6s %-4s %s\n' NAME EXPECT GOT BANK VOX CLS RESULT
for k in $ORDER; do
  w="${WORD[$k]}"
  out=$($RUN "imasm derive $w" "banked $w" "vox verdict $w" "vox classify $w" 2>/dev/null)
  got=$(echo "$out" | grep -o 'crystal: [0-9]*' | head -1 | grep -o '[0-9]*')
  bank=$(echo "$out" | grep -qE 'OK —' && echo OK || echo NO)
  vox=$(echo "$out" | grep -oE 'verdict [TBNF]' | awk '{print $2}' | head -1)
  cls=$(echo "$out" | grep -F "$w " | awk '{print $NF}' | tail -1)
  got=${got//[$'\r\n ']/}; bank=${bank//[$'\r\n ']/}; vox=${vox//[$'\r\n ']/}; cls=${cls//[$'\r\n ']/}
  res=PASS
  [ "$got" = "${EXPECT[$k]}" ] && [ "$bank" = OK ] && [ "$vox" = T ] && [ "$cls" = "⊞" ] || { res=FAIL; fail=1; }
  printf '%-16s %-10s %-10s %-6s %-6s %-4s %s\n' "$k" "${EXPECT[$k]}" "$got" "$bank" "$vox" "$cls" "$res"
done

echo
echo "--- proved controls (Mills, Lee-Yang): crystal 3444190, vox T, classify ⊞ ---"
declare -A CTL=(
 [mills_affirm]="⊢∈≻⊤⋈⊡≺⊥⊞∋⊙⋈≻⊤⊡⊣" [mills_negate]="⊢⊙≻⋈∈⊤⊥⊞≺∋⊡⊣⊙"
 [leeyang_affirm]="⊢∈≻⊤≺⊥⊞⋈∋⊙⊡⊣" [leeyang_negate]="⊢⊣∈≻⊤⋈≺⊥⊙⊞∋⊡⊣"
)
printf '%-16s %-10s %-10s %-6s %-4s %s\n' NAME EXPECT GOT VOX CLS RESULT
for k in mills_affirm mills_negate leeyang_affirm leeyang_negate; do
  w="${CTL[$k]}"
  out=$($RUN "imasm derive $w" "vox verdict $w" "vox classify $w" 2>/dev/null)
  got=$(echo "$out" | grep -o 'crystal: [0-9]*' | head -1 | grep -o '[0-9]*')
  vox=$(echo "$out" | grep -oE 'verdict [TBNF]' | awk '{print $2}' | head -1)
  cls=$(echo "$out" | grep -F "$w " | awk '{print $NF}' | tail -1)
  got=${got//[$'\r\n ']/}; vox=${vox//[$'\r\n ']/}; cls=${cls//[$'\r\n ']/}
  res=PASS
  [ "$got" = 3444190 ] && [ "$vox" = T ] && [ "$cls" = "⊞" ] || { res=FAIL; fail=1; }
  printf '%-16s %-10s %-10s %-6s %-4s %s\n' "$k" 3444190 "$got" "$vox" "$cls" "$res"
done
echo
echo "--- crystal decodes: descent 3444190 vs fork 16404190 differ only at ⊢ ---"
$RUN "crystal 3444190" "crystal 16404190" 2>/dev/null | grep -E '⊢:' | sed 's/^ */  /'

echo
[ $fail -eq 0 ] && echo "CENSUS: PASS — all ten objects at expected crystals, bank OK, vox T, classify ⊞" \
               || { echo "CENSUS: FAIL"; exit 1; }
