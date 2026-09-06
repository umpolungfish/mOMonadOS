# Glue wrapped big-integer lines onto an incomplete bigint verb.
# Must match mOMonadOS/src/repl.rs (BIGINT_CONTINUATION_DIGITS + awaiting_bigint_arg).
#
# Only digit lines that arrive while a hold is open are joined. A lone short
# digit line after an unrelated command (e.g. `tick` then `500`) stays its own
# command — that was the cross-argv / cross-line glue bug.
#
#   prime_winding factor
#   2211…870
#   9229…199
#   12
# → one line: prime_winding factor 2211…8709229…199 12
#
# Usage: … | awk -f join_digit_continuations.awk
# For multiple argv commands, run once per argv so holds never cross commands.

function awaiting(line,    n, a) {
  n = split(line, a, /[[:space:]]+/)
  if (n == 1 && a[1] == "gpu_ecm") return 1
  if (n == 2 && a[1] == "gpu_ecm" && a[2] == "bsgs") return 1
  if (n == 1 && a[1] == "gpu_gnfs") return 1
  if (n == 1 && a[1] == "gpu_factor") return 1
  if (n == 2 && a[1] == "gpu_rho" && a[2] == "factor") return 1
  if (n == 2 && (a[1] == "nested_oneshot" || a[1] == "nested" || a[1] == "nos") && a[2] == "factor") return 1
  if (n == 2 && (a[1] == "doubly_nested_oneshot" || a[1] == "dnos") && a[2] == "factor") return 1
  if (n == 2 && a[1] == "prime_winding" && a[2] == "factor") return 1
  if (n == 2 && (a[1] == "trilattice_factor" || a[1] == "tfactor") && a[2] == "factor") return 1
  if (n == 2 && a[1] == "winding" && a[2] == "factor") return 1
  return 0
}

/^[0-9]+$/ {
  if (holding) {
    if (length($0) >= 16 && buf ~ /[0-9]$/) buf = buf $0
    else buf = buf " " $0
    next
  }
  if (length(buf)) { print buf; buf = "" }
  print
  next
}
{
  if (holding) {
    print buf
    buf = $0
    holding = awaiting($0)
    next
  }
  if (length(buf)) print buf
  buf = $0
  holding = awaiting($0)
}
END {
  if (length(buf)) print buf
}
