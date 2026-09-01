#!/usr/bin/env python3
"""Chart from gpu_sixteen3_tensor_kernel_scaling.csv -- real timing from
`gpu_sixteen3_tensor_kernel <n>` runs on the RTX 4070, one boot per row so
CUDA context/module setup happens once and every number below is the
module's own internal Instant timing, not shell wall-clock including boot."""
import csv
import matplotlib.pyplot as plt

rows = []
with open("gpu_sixteen3_tensor_kernel_scaling.csv") as f:
    for row in csv.DictReader(f):
        rows.append({k: float(v) for k, v in row.items()})

n = [r["n"] for r in rows]
gen = [r["gen_ms"] for r in rows]
gpu = [r["htod_launch_sync_ms"] for r in rows]
dtoh = [r["dtoh_ms"] for r in rows]
verify = [r["cpu_verify_ms"] for r in rows]
total = [r["total_ms"] for r in rows]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(13, 5.2))

ax1.plot(n, gen, "o-", label="CPU: generate random triples")
ax1.plot(n, gpu, "o-", label="GPU: htod + kernel launch + sync")
ax1.plot(n, dtoh, "o-", label="GPU->CPU: dtoh readback")
ax1.plot(n, verify, "o-", label="CPU: verify vs imasm_core (3 stages)")
ax1.plot(n, total, "ko--", label="total", linewidth=1.5)
ax1.set_xscale("log")
ax1.set_yscale("log")
ax1.set_xlabel("register triples (n)")
ax1.set_ylabel("elapsed (ms)")
ax1.set_title("gpu_sixteen3_tensor_kernel: where the time goes")
ax1.legend(fontsize=8, loc="upper left")
ax1.grid(True, which="both", alpha=0.25)

gpu_throughput = [ni / (gi / 1000.0) for ni, gi in zip(n, gpu)]
total_throughput = [ni / (ti / 1000.0) for ni, ti in zip(n, total)]
ax2.plot(n, gpu_throughput, "o-", color="tab:orange", label="GPU stage alone (htod+kernel+sync)")
ax2.plot(n, total_throughput, "ko-", label="end to end, including CPU verify of every result")
ax2.set_xscale("log")
ax2.set_yscale("log")
ax2.set_xlabel("register triples (n)")
ax2.set_ylabel("triples / second")
ax2.set_title("throughput: GPU compute vs the CPU check that verifies it")
ax2.legend(fontsize=8, loc="upper left")
ax2.grid(True, which="both", alpha=0.25)

fig.suptitle(
    "gpu_sixteen3_tensor_kernel: chained union -> meet_t -> truth_swap on the RTX 4070\n"
    "one boot per point, internal Instant timing, 10^4 to 10^9 register triples",
    fontsize=10,
)
fig.tight_layout(rect=[0, 0, 1, 0.93])
fig.savefig("gpu_sixteen3_tensor_kernel_scaling.png", dpi=150)
print("wrote gpu_sixteen3_tensor_kernel_scaling.png")
