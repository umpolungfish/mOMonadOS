#!/usr/bin/env python3
"""Two charts from real `gpu_sixteen3_tensor_kernel <n>` runs on the RTX
4070, one boot per n so CUDA context/module setup happens once and every
number is the module's own internal Instant timing, not shell wall-clock
including boot.

BEFORE (gpu_sixteen3_tensor_kernel_scaling_before_gpu_verify.csv): the
original module, verification a single-threaded CPU loop reading every
GPU result back and recomputing it against imasm_core.

AFTER (gpu_sixteen3_tensor_kernel_scaling.csv): verification moved onto
the GPU -- every thread also computes the same three stages via an
independently-written flat kernel and folds disagreement into one atomic
counter per stage, O(N) work that never leaves the device. A small,
fixed-size sample (10,000 entries regardless of n) still gets checked
directly against the real CPU imasm_core functions, the ground-truth
control a GPU-only self-consistency check cannot be.
"""
import csv
import matplotlib.pyplot as plt

def load(path):
    rows = []
    with open(path) as f:
        for row in csv.DictReader(f):
            rows.append({k: float(v) for k, v in row.items() if k != "control_k"})
    return rows

before = load("gpu_sixteen3_tensor_kernel_scaling_before_gpu_verify.csv")
after = load("gpu_sixteen3_tensor_kernel_scaling.csv")

n_b = [r["n"] for r in before]
total_b = [r["total_ms"] for r in before]
verify_b = [r["cpu_verify_ms"] for r in before]
gpu_b = [r["htod_launch_sync_ms"] for r in before]

n_a = [r["n"] for r in after]
total_a = [r["total_ms"] for r in after]
gen_a = [r["gen_ms"] for r in after]
gpu_a = [r["htod_launch_sync_ms"] for r in after]
verify_a = [r["dtoh_counters_ms"] + r["cpu_control_ms"] for r in after]

fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(13, 5.2))

ax1.plot(n_b, total_b, "o--", color="tab:red", label="BEFORE: total (CPU-loop verify)")
ax1.plot(n_b, verify_b, "o:", color="tab:red", alpha=0.6, label="BEFORE: CPU verify loop alone")
ax1.plot(n_a, total_a, "o-", color="tab:blue", label="AFTER: total (GPU verify)")
ax1.plot(n_a, gen_a, "s-", color="tab:cyan", alpha=0.7, label="AFTER: host RNG generation (new bottleneck)")
ax1.plot(n_a, verify_a, "o-", color="tab:green", label="AFTER: verify (GPU counters + CPU control)")
ax1.set_xscale("log")
ax1.set_yscale("log")
ax1.set_xlabel("register triples (n)")
ax1.set_ylabel("elapsed (ms)")
ax1.set_title("moving verification onto the GPU")
ax1.legend(fontsize=7.5, loc="upper left")
ax1.grid(True, which="both", alpha=0.25)

speedup = [b / a for b, a in zip(total_b, total_a)]
ax2.plot(n_a, speedup, "o-", color="tab:purple")
ax2.set_xscale("log")
ax2.set_xlabel("register triples (n)")
ax2.set_ylabel("total wall time, before / after")
ax2.set_title("speedup from moving the O(N) verify loop onto the GPU")
ax2.grid(True, which="both", alpha=0.25)
ax2.axhline(1.0, color="gray", linewidth=0.8)
for x, y in zip(n_a, speedup):
    ax2.annotate(f"{y:.1f}x", (x, y), textcoords="offset points", xytext=(0, 8), fontsize=8, ha="center")

fig.suptitle(
    "gpu_sixteen3_tensor_kernel: chained union -> meet_t -> truth_swap on the RTX 4070\n"
    "one boot per point, internal Instant timing, 10^4 to 10^9 register triples",
    fontsize=10,
)
fig.tight_layout(rect=[0, 0, 1, 0.93])
fig.savefig("gpu_sixteen3_tensor_kernel_scaling.png", dpi=150)
print("wrote gpu_sixteen3_tensor_kernel_scaling.png")
for x, y in zip(n_a, speedup):
    print(f"  n={int(x):>12}  {y:.2f}x")
