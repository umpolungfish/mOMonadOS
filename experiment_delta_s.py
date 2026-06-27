#!/usr/bin/env python3
"""
experiment_delta_s.py — Thermal entropy production during mOMonadOS execution

Does running mOMonadOS produce a measurable ΔS above idle baseline?

Physical model
--------------
  CPU   : Intel i9-9900K, TDP_max = 95 W
  T_op  : 330 K  (conservative operating estimate; no RAPL in WSL2)
  source: /proc/stat jiffies → system-wide CPU utilization fraction f
  Q     : f × TDP × Δt   (heat dissipated per sampling interval)
  ΔS    : Σ Q / T_op      (total entropy produced, J/K)

Landauer lower bound
--------------------
  Each irreversible bit erasure dissipates ≥ k_B T ln(2).
  We estimate instruction count from QEMU CPU% × clock, use it as an upper
  bound on bit erasures (1 erasure per retired instruction is conservative).

Experiment phases
-----------------
  IDLE_PRE    — 20 s no QEMU (true system idle)
  BOOT_ONLY   — one QEMU boot with '\nquit\n' (infrastructure overhead)
  COMPUTATION — N trials of '\nrun K\nquit\n' (IG token execution)
  IDLE_POST   — 15 s no QEMU (recovery check)

Three-way comparison:
  idle baseline  vs  QEMU boot overhead  vs  mOMonadOS IG computation
"""

import os, sys, time, json, subprocess, threading, textwrap
from pathlib import Path
import psutil

# ── Physical constants ────────────────────────────────────────────────────────
TDP_W         = 95.0           # i9-9900K whole-chip TDP (watts)
N_CORES       = psutil.cpu_count(logical=True)
# Per-core power: TDP is not divided linearly but 1-core active ≈ TDP/N_cores
# is a standard conservative estimate when other cores are near-idle.
CORE_W        = TDP_W / N_CORES   # ~11.875 W per logical core
T_OP_K        = 330.0          # CPU operating temperature estimate (K)
K_B           = 1.380649e-23   # Boltzmann constant (J/K)
CPU_GHZ       = 3.6            # base clock

# ── Experiment parameters ─────────────────────────────────────────────────────
IDLE_PRE_S    = 20             # seconds of idle baseline before any QEMU
IDLE_POST_S   = 15             # seconds of idle after last trial
SAMPLE_HZ     = 20             # CPU samples per second
SAMPLE_DT     = 1.0 / SAMPLE_HZ
RUN_TICKS     = 2_000_000      # IG ticks per trial — long enough for stable sampling
N_TRIALS      = 5              # computation trials per tier

# Tier-stratified commands
#   O1_BOOTSTRAP  — default boot sequence (IMSCRIB→AREV→FSPLIT→...→IMSCRIB), O_1
#   O1_COMPOUND   — Verticullum compound (ENGAGR→FSPLIT→EVALT→FFUSE→...), O_1 with b_live>0
#   OINF_CHAIN    — Verticullum (500k, builds b_live) then XIV_Tier_Climber (1.5M, O_∞)
CMD_O1_BOOTSTRAP = f"\nrun {RUN_TICKS}\nquit\n"
CMD_O1_COMPOUND  = f"\ncompound load Verticullum\nrun {RUN_TICKS}\nquit\n"
CMD_OINF_CHAIN   = f"\ncompound load Verticullum\nrun 500000\nload XIV\nrun 1500000\nquit\n"

MOMONADOS_DIR = Path(__file__).parent
ELF           = MOMONADOS_DIR / "target/x86_64-unknown-none/release/momonados"

QEMU_CMD = [
    "qemu-system-x86_64",
    "-kernel", str(ELF),
    "-m", "256M",
    "-display", "none",
    "-no-reboot",
    "-device", "isa-debug-exit,iobase=0xf4,iosize=4",
    "-serial", "stdio",
]

# ── CPU sampling ──────────────────────────────────────────────────────────────
def read_stat():
    with open("/proc/stat") as f:
        tok = f.readline().split()[1:]
    t = tuple(int(x) for x in tok[:8])
    return t   # user nice sys idle iowait irq soft steal

def cpu_util(s0, s1):
    tot = sum(s1) - sum(s0)
    idle = (s1[3] + s1[4]) - (s0[3] + s0[4])
    return 0.0 if tot <= 0 else max(0.0, 1.0 - idle / tot)

class CPUSampler:
    """
    Dual-channel CPU sampler:
      system_util — /proc/stat jiffies (whole machine, noisy background included)
      proc_cpu    — psutil per-process CPU% for QEMU only (background-isolated)

    PRIMARY signal for ΔS computation is proc_cpu × CORE_W, not system-wide util.
    System util is recorded for context/diagnostics only.
    """

    def __init__(self):
        self.records   = []   # (t, sys_util, proc_cpu_pct)
        self.qemu_pid  = None
        self._stop     = threading.Event()
        self._t        = None
        self._qproc    = None

    def start(self):
        self._stop.clear()
        self._t = threading.Thread(target=self._loop, daemon=True)
        self._t.start()

    def stop(self):
        self._stop.set()
        self._t.join(timeout=5)

    def _qcpu(self):
        """Per-process CPU% for QEMU — unaffected by other machine load."""
        if self.qemu_pid is None:
            return 0.0
        try:
            if self._qproc is None or self._qproc.pid != self.qemu_pid:
                self._qproc = psutil.Process(self.qemu_pid)
                self._qproc.cpu_percent(interval=None)  # prime
                return 0.0
            return self._qproc.cpu_percent(interval=None)
        except (psutil.NoSuchProcess, psutil.AccessDenied):
            return 0.0

    def _loop(self):
        s_prev = read_stat()
        while not self._stop.is_set():
            time.sleep(SAMPLE_DT)
            s_now  = read_stat()
            t_now  = time.monotonic()
            sys_u  = cpu_util(s_prev, s_now)
            proc_u = self._qcpu()
            self.records.append((t_now, sys_u, proc_u))
            s_prev = s_now

    # ── Stats ─────────────────────────────────────────────────────────────────
    def stats(self, label):
        if not self.records:
            return {"label": label, "n": 0}
        dur      = self.records[-1][0] - self.records[0][0]
        sys_u    = [r[1] for r in self.records]
        proc_u   = [r[2] for r in self.records]

        mean_sys  = sum(sys_u)  / len(sys_u)
        mean_proc = sum(proc_u) / len(proc_u)

        # PRIMARY: per-process heat — CORE_W × (proc_cpu% / 100) per interval
        # proc_cpu can exceed 100% on multi-threaded QEMU (guest + host threads);
        # cap at N_CORES × 100 to stay physical.
        proc_fracs = [min(p, N_CORES * 100) / 100.0 for p in proc_u]
        Q_proc  = sum(f * CORE_W * SAMPLE_DT for f in proc_fracs)
        dS_proc = Q_proc / T_OP_K

        # SECONDARY: system-wide (for comparison / diagnostics only)
        Q_sys   = sum(u * TDP_W * SAMPLE_DT for u in sys_u)
        dS_sys  = Q_sys / T_OP_K

        n_instr = int(mean_proc / 100 * CPU_GHZ * 1e9 * dur)
        land_dS = K_B * n_instr * 0.693147

        return {
            "label":             label,
            "n":                 len(self.records),
            "duration_s":        round(dur, 3),
            # primary (process-isolated)
            "mean_qemu_cpu_pct": round(mean_proc, 2),
            "total_Q_proc_J":    round(Q_proc, 5),
            "delta_S_proc_JK":   round(dS_proc, 8),
            "delta_S_proc_kB":   round(dS_proc / K_B, 2),
            # secondary (system-wide, noisy)
            "mean_sys_util":     round(mean_sys, 5),
            "total_Q_sys_J":     round(Q_sys, 4),
            "delta_S_sys_JK":    round(dS_sys, 7),
            # Landauer
            "est_instructions":  n_instr,
            "landauer_dS_JK":    round(land_dS, 10),
            "landauer_dS_kB":    round(land_dS / K_B, 2),
        }

# ── QEMU runner ───────────────────────────────────────────────────────────────
def run_qemu(stdin_script: str, sampler: CPUSampler, timeout=120):
    """
    Run QEMU, feed stdin_script to the serial port, collect CPU samples.
    Returns (output_text, duration_s, retcode).
    """
    proc = subprocess.Popen(
        QEMU_CMD,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        cwd=str(MOMONADOS_DIR),
    )

    # Register PID with sampler for per-process CPU tracking
    sampler.qemu_pid = proc.pid
    # Prime psutil counter before sending input
    try:
        p = psutil.Process(proc.pid)
        p.cpu_percent(interval=None)
    except Exception:
        pass

    t0 = time.monotonic()
    try:
        out, _ = proc.communicate(input=stdin_script.encode(), timeout=timeout)
    except subprocess.TimeoutExpired:
        proc.kill()
        out, _ = proc.communicate()
        out = (out or b"") + b"\n[TIMEOUT]"
    dur = time.monotonic() - t0
    sampler.qemu_pid = None
    return out.decode(errors="replace"), dur, proc.returncode

# ── Reporting helpers ─────────────────────────────────────────────────────────
def fmt(st):
    qcpu = st.get("mean_qemu_cpu_pct", 0)
    dsp  = st.get("delta_S_proc_JK", 0)
    dss  = st.get("delta_S_sys_JK",  st.get("delta_S_JK", 0))
    sys_u = st.get("mean_sys_util", st.get("mean_util", 0))
    return (
        f"  dur={st['duration_s']:.2f}s  "
        f"QEMU={qcpu:.1f}%  "
        f"ΔS_proc={dsp:.3e}J/K ({dsp/K_B:.2e}k_B)  "
        f"[sys_util={sys_u*100:.1f}%  ΔS_sys={dss:.3e}J/K]"
    )

def excess_proc(trial_st, boot_st):
    """
    Process-isolated excess: computation trial minus boot-only.
    Both measured by QEMU process CPU%, so background is cancelled.
    """
    # Normalise boot to the trial duration to get a fair per-second rate
    boot_rate = boot_st["delta_S_proc_JK"] / max(boot_st["duration_s"], 1e-6)
    base_equiv = boot_rate * trial_st["duration_s"]
    ex = trial_st["delta_S_proc_JK"] - base_equiv
    return ex, ex / K_B

# ── Main experiment ───────────────────────────────────────────────────────────
def run():
    if not ELF.exists():
        sys.exit(f"ERROR: ELF not found at {ELF}\nRun: bash build_bootimage.sh release")

    results = {
        "hardware": f"Intel i9-9900K @ {CPU_GHZ}GHz, TDP={TDP_W}W, {N_CORES} logical cores",
        "T_op_K": T_OP_K, "K_B": K_B,
        "run_ticks": RUN_TICKS, "n_trials": N_TRIALS,
        "phases": {},
    }

    sep = "=" * 64

    # ── IDLE PRE-BASELINE ─────────────────────────────────────────────────────
    print(f"\n{sep}\nPHASE 1 — IDLE BASELINE  ({IDLE_PRE_S}s, no QEMU)\n{sep}")
    sp = CPUSampler(); sp.start()
    for i in range(IDLE_PRE_S):
        time.sleep(1)
        if (i+1) % 5 == 0: print(f"  {i+1}/{IDLE_PRE_S}s ...")
    sp.stop()
    idle_st = sp.stats("idle_pre")
    results["phases"]["idle_pre"] = idle_st
    print(fmt(idle_st))
    # idle QEMU CPU% is 0 — background context only; use sys rate for diagnostics
    idle_sys_rate = idle_st["delta_S_sys_JK"] / idle_st["duration_s"]
    print(f"  [background sys ΔS rate: {idle_sys_rate:.4e} J/K/s  "
          f"(other processes — not used in primary signal)]")

    # ── BOOT-ONLY CONTROL ─────────────────────────────────────────────────────
    print(f"\n{sep}\nPHASE 2 — BOOT-ONLY CONTROL  (QEMU boots, immediately quits)\n{sep}")
    sp = CPUSampler(); sp.start()
    out, dur, rc = run_qemu("\nquit\n", sp)
    sp.stop()
    boot_st = sp.stats("boot_only")
    boot_st["qemu_rc"] = rc; boot_st["qemu_dur"] = round(dur, 3)
    results["phases"]["boot_only"] = boot_st
    print(fmt(boot_st))
    print(f"  boot ΔS_proc (isolated): {boot_st['delta_S_proc_JK']:.4e} J/K  "
          f"({boot_st['delta_S_proc_kB']:.2e} k_B)")

    def run_trial_set(label, cmd, n, phase_num):
        print(f"\n{sep}\nPHASE {phase_num} — {label}  ({n} × {RUN_TICKS} ticks)\n{sep}")
        trials = []
        for i in range(n):
            print(f"\n  Trial {i+1}/{n} ...")
            sp = CPUSampler(); sp.start()
            out, dur, rc = run_qemu(cmd, sp)
            sp.stop()
            st = sp.stats(f"{label}_{i+1}")
            st["qemu_rc"] = rc; st["qemu_dur"] = round(dur, 3)
            tick_line = next((l for l in out.splitlines() if "Tick:" in l), "")
            tier_line = next((l for l in out.splitlines() if "Tier:" in l or "Frob:" in l), "")
            st["output_tick"] = tick_line.strip()
            st["frob_line"]   = tier_line.strip()
            comp_ex_J, comp_ex_kB = excess_proc(st, boot_st)
            st["comp_excess_proc_JK"] = round(comp_ex_J, 8)
            st["comp_excess_proc_kB"] = round(comp_ex_kB, 2)
            trials.append(st)
            print(fmt(st))
            if tick_line: print(f"  {tick_line.strip()}")
            if tier_line: print(f"  {tier_line.strip()}")
            time.sleep(1)
        return trials

    # Three tier levels:
    o1_bootstrap = run_trial_set("O1_BOOTSTRAP",  CMD_O1_BOOTSTRAP, N_TRIALS, 3)
    o1_compound  = run_trial_set("O1_COMPOUND",   CMD_O1_COMPOUND,  N_TRIALS, 4)
    oinf_chain   = run_trial_set("OINF_CHAIN",    CMD_OINF_CHAIN,   N_TRIALS, 5)

    results["phases"]["o1_bootstrap"] = o1_bootstrap
    results["phases"]["o1_compound"]  = o1_compound
    results["phases"]["oinf_chain"]   = oinf_chain

    # ── IDLE POST-RECOVERY ────────────────────────────────────────────────────
    print(f"\n{sep}\nPHASE 6 — RECOVERY  ({IDLE_POST_S}s idle)\n{sep}")
    sp = CPUSampler(); sp.start()
    for i in range(IDLE_POST_S):
        time.sleep(1)
        if (i+1) % 5 == 0: print(f"  {i+1}/{IDLE_POST_S}s ...")
    sp.stop()
    rec_st = sp.stats("idle_post")
    results["phases"]["idle_post"] = rec_st
    print(fmt(rec_st))

    # ── ANALYSIS ──────────────────────────────────────────────────────────────
    print(f"\n{sep}\nANALYSIS — TIER-STRATIFIED ΔS\n{sep}")

    def mean_rate(trials):
        mean_dS  = sum(t["delta_S_proc_JK"] for t in trials) / len(trials)
        mean_dur = sum(t["qemu_dur"]         for t in trials) / len(trials)
        return mean_dS / max(mean_dur, 1e-9), mean_dS, mean_dur

    def mean_qcpu(trials):
        return sum(t["mean_qemu_cpu_pct"] for t in trials) / len(trials)

    r1, dS1, dur1 = mean_rate(o1_bootstrap)
    r2, dS2, dur2 = mean_rate(o1_compound)
    r3, dS3, dur3 = mean_rate(oinf_chain)

    boot_dS   = boot_st["delta_S_proc_JK"]
    boot_rate = boot_dS / max(boot_st["duration_s"], 1e-9)
    rec_sys_ratio = (rec_st["mean_sys_util"] / idle_st["mean_sys_util"]
                     if idle_st.get("mean_sys_util", 0) > 0 else 1.0)

    print(f"\n  Method: PER-PROCESS CPU% × CORE_W={CORE_W:.2f}W  (background-isolated)")
    print(f"  [Background noise from other processes NOT included]\n")
    print(f"  {'Label':<22}  {'QEMU%':>6}  {'ΔS/trial':>12}  {'ΔS rate':>14}  {'vs O₁':>8}")
    print(f"  {'-'*70}")
    print(f"  {'Boot-only (ctrl)':<22}  {'—':>6}  {boot_dS:>12.4e}  {boot_rate:>14.4e}  {'—':>8}")
    print(f"  {'O₁ Bootstrap':<22}  {mean_qcpu(o1_bootstrap):>6.1f}  {dS1:>12.4e}  {r1:>14.4e}  {'ref':>8}")
    print(f"  {'O₁ Compound (b_live)':<22}  {mean_qcpu(o1_compound):>6.1f}  {dS2:>12.4e}  {r2:>14.4e}  {r2/r1:>8.4f}×")
    print(f"  {'O_∞ Chain (Vert→XIV)':<22}  {mean_qcpu(oinf_chain):>6.1f}  {dS3:>12.4e}  {r3:>14.4e}  {r3/r1:>8.4f}×")

    print(f"\n  ΔS rate comparison (process-isolated, J/K/s):")
    print(f"    O_∞ / O₁_bootstrap = {r3/r1:.5f}×")
    print(f"    O₁_compound / O₁_bootstrap = {r2/r1:.5f}×")
    print(f"    O_∞ / O₁_compound  = {r3/r2:.5f}×")

    thr = 0.05  # 5% threshold
    print(f"\n  Significance threshold: {thr*100:.0f}%  (|ratio - 1.0| > {thr:.2f})")
    for label, ratio in [("O_∞ vs O₁_bootstrap", r3/r1), ("O₁_compound vs O₁_bootstrap", r2/r1), ("O_∞ vs O₁_compound", r3/r2)]:
        if abs(ratio - 1.0) > thr:
            direction = "MORE" if ratio > 1.0 else "LESS"
            print(f"  SIGNIFICANT: {label} = ×{ratio:.4f} — {direction} entropy at {abs(ratio-1)*100:.1f}% above threshold")
        else:
            print(f"  NOT SIGNIFICANT: {label} = ×{ratio:.4f} — within noise")

    print(f"\n  Background sys_util recovery: ×{rec_sys_ratio:.3f} vs pre-experiment idle")

    analysis = {
        "method":           "per-process CPU% × CORE_W (background-isolated)",
        "CORE_W":           CORE_W,
        "boot_dS_proc_JK":  round(boot_dS, 8),
        "boot_rate_JK_s":   round(boot_rate, 8),
        "o1_bootstrap":     {"mean_dS_JK": round(dS1,8), "rate_JK_s": round(r1,8), "mean_dur_s": round(dur1,3), "mean_qcpu": round(mean_qcpu(o1_bootstrap),2)},
        "o1_compound":      {"mean_dS_JK": round(dS2,8), "rate_JK_s": round(r2,8), "mean_dur_s": round(dur2,3), "mean_qcpu": round(mean_qcpu(o1_compound),2), "b_live_present": True},
        "oinf_chain":       {"mean_dS_JK": round(dS3,8), "rate_JK_s": round(r3,8), "mean_dur_s": round(dur3,3), "mean_qcpu": round(mean_qcpu(oinf_chain),2), "note": "Verticullum 500k + XIV_Tier_Climber 1.5M"},
        "rate_ratio_oinf_vs_o1_bootstrap": round(r3/r1, 6),
        "rate_ratio_o1c_vs_o1b":           round(r2/r1, 6),
        "rate_ratio_oinf_vs_o1c":          round(r3/r2, 6),
        "recovery_sys_util_ratio":         round(rec_sys_ratio, 4),
    }
    results["analysis"] = analysis

    out_path = MOMONADOS_DIR / "delta_s_results.json"
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\n  Full results → {out_path}")
    return results


if __name__ == "__main__":
    run()
