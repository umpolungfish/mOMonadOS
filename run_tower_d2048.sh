#!/usr/bin/env bash
# Run one PARI tower distillation level for d=2048 (host-side; mOMonadOS banks results in d2048_sic.rs)
set -euo pipefail
cd "$(dirname "$0")"
LEVEL="${1:-4}"
TIMEOUT="${2:-600}"
OUT="../d12_sic_build/tower_C${LEVEL}.poly"
rm -f "$OUT"
echo "=== d2048 tower distillation: C${LEVEL} (timeout ${TIMEOUT}s) ==="
cat > /tmp/tower_d2048_run.gp <<GP
default(parisize, 2000000000);
default(parisizemax, 24000000000);
md = 4190205; n = ${LEVEL};
print("[tower_d2048] C", n, " over F = Q(sqrt ", md, ")");
F = bnfinit(y^2 - md, 1);
bnr = bnrinit(F, 1);
print("class group ", bnr.cyc, " order ", bnr.no);
t0 = getwalltime();
H = bnrclassfield(bnr, [n], 2);
dt = getwalltime() - t0;
C = H[1]; if(type(C) != "t_POL", C = H);
write("${OUT}", C);
print("[done] deg/Q=", poldegree(C), " deg/F=", poldegree(C)/2, " ms=", dt);
print("disc exponent over F: m_d^", poldegree(C)/2);
GP
timeout "$TIMEOUT" gp -q /tmp/tower_d2048_run.gp 2>&1 | grep -vE 'parisize|Warning' || {
  echo "TIMEOUT at C${LEVEL} — retry: ./run_tower_d2048.sh ${LEVEL} 1800"
  exit 124
}
echo "=== written ${OUT} ==="