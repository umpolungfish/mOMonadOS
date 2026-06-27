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

    # ── COMPUTATION TRIALS ────────────────────────────────────────────────────
    print(f"\n{sep}\nPHASE 3 — mOMonadOS COMPUTATION  ({N_TRIALS} × run {RUN_TICKS})\n{sep}")
    trial_results = []
    cmd_script = f"\nrun {RUN_TICKS}\nquit\n"

    for i in range(N_TRIALS):
        print(f"\n  Trial {i+1}/{N_TRIALS} ...")
        sp = CPUSampler(); sp.start()
        out, dur, rc = run_qemu(cmd_script, sp)
        sp.stop()
        st = sp.stats(f"trial_{i+1}")
        st["qemu_rc"] = rc; st["qemu_dur"] = round(dur, 3)

        # Capture tick count and tier from output
        tick_line = next((l for l in out.splitlines() if "Tick:" in l), "")
        tier_line = next((l for l in out.splitlines() if "Tier:" in l or "Frob:" in l), "")
        st["output_tick"] = tick_line.strip()
        st["frob_line"]   = tier_line.strip()

        # Process-isolated excess: computation above boot-only rate
        comp_ex_J, comp_ex_kB = excess_proc(st, boot_st)
        st["comp_excess_proc_JK"] = round(comp_ex_J, 8)
        st["comp_excess_proc_kB"] = round(comp_ex_kB, 2)

        trial_results.append(st)
        print(fmt(st))
        print(f"  comp excess vs boot (isolated): {comp_ex_J:.4e} J/K  ({comp_ex_kB:.2e} k_B)")
        if tick_line: print(f"  {tick_line.strip()}")
        if tier_line: print(f"  {tier_line.strip()}")
        time.sleep(1)

    results["phases"]["computation_trials"] = trial_results

    # ── IDLE POST-RECOVERY ────────────────────────────────────────────────────
    print(f"\n{sep}\nPHASE 4 — RECOVERY  ({IDLE_POST_S}s idle)\n{sep}")
    sp = CPUSampler(); sp.start()
    for i in range(IDLE_POST_S):
        time.sleep(1)
        if (i+1) % 5 == 0: print(f"  {i+1}/{IDLE_POST_S}s ...")
    sp.stop()
    rec_st = sp.stats("idle_post")
    results["phases"]["idle_post"] = rec_st
    print(fmt(rec_st))

    # ── ANALYSIS ──────────────────────────────────────────────────────────────
    print(f"\n{sep}\nANALYSIS\n{sep}")

    mean_comp  = sum(t["comp_excess_proc_JK"] for t in trial_results) / N_TRIALS
    mean_dS    = sum(t["delta_S_proc_JK"]    for t in trial_results) / N_TRIALS
    mean_dur   = sum(t["qemu_dur"]           for t in trial_results) / N_TRIALS
    mean_qcpu  = sum(t["mean_qemu_cpu_pct"]  for t in trial_results) / N_TRIALS
    mean_land  = sum(t["landauer_dS_JK"]     for t in trial_results) / N_TRIALS
    mean_instr = sum(t["est_instructions"]   for t in trial_results) / N_TRIALS

    boot_dS    = boot_st["delta_S_proc_JK"]
    rec_sys_ratio = (rec_st["mean_sys_util"] / idle_st["mean_sys_util"]
                     if idle_st.get("mean_sys_util", 0) > 0 else 1.0)

    # Boot-normalised per-second ΔS rates (process-isolated)
    boot_rate  = boot_dS / max(boot_st["duration_s"], 1e-9)
    trial_rate = mean_dS / max(mean_dur, 1e-9)
    rate_ratio = trial_rate / boot_rate if boot_rate > 0 else float("inf")

    print(f"\n  Method: PER-PROCESS CPU% × CORE_W={CORE_W:.2f}W  (background-isolated)")
    print(f"  [Background noise from other processes is NOT included in ΔS_proc]\n")
    print(f"  Boot-only ΔS_proc:          {boot_dS:.4e} J/K  ({boot_dS/K_B:.2e} k_B)")
    print(f"  Boot ΔS rate:               {boot_rate:.4e} J/K/s")
    print(f"\n  Mean trial duration:        {mean_dur:.2f}s")
    print(f"  Mean QEMU CPU%:             {mean_qcpu:.1f}%")
    print(f"  Mean trial ΔS_proc:         {mean_dS:.4e} J/K  ({mean_dS/K_B:.2e} k_B)")
    print(f"  Mean trial ΔS rate:         {trial_rate:.4e} J/K/s")
    print(f"  Trial rate / boot rate:     ×{rate_ratio:.3f}")
    print(f"\n  Mean comp excess (proc):    {mean_comp:.4e} J/K  ({mean_comp/K_B:.2e} k_B)")
    print(f"  Mean est. instructions:     {mean_instr:.3e}")
    print(f"  Mean Landauer lower bound:  {mean_land:.4e} J/K  ({mean_land/K_B:.2e} k_B)")
    if mean_land > 0 and mean_dS > 0:
        print(f"  Actual / Landauer ratio:    ×{mean_dS/mean_land:.2e}")

    print()
    # Significance: is per-second ΔS rate during computation different from boot?
    if rate_ratio > 1.10:
        print(f"  VERDICT: Computation ΔS rate is ×{rate_ratio:.3f} boot rate.")
        print(f"           The IG token execution generates MORE heat per second than")
        print(f"           QEMU boot overhead alone. mOMonadOS computation is thermally")
        print(f"           distinguishable from its own infrastructure cost.")
    elif rate_ratio < 0.90:
        print(f"  VERDICT: Computation ΔS rate is ×{rate_ratio:.3f} boot rate.")
        print(f"           The IG computation generates LESS heat per second than QEMU boot.")
        print(f"           The Frobenius structure may be doing real work here.")
    else:
        print(f"  VERDICT: Computation ΔS rate ≈ boot rate (×{rate_ratio:.3f}).")
        print(f"           Cannot distinguish computation from infrastructure cost.")

    print(f"\n  Background sys_util recovery: ×{rec_sys_ratio:.3f} vs pre-experiment idle")

    analysis = {
        "method":                "per-process CPU% × CORE_W (background-isolated)",
        "CORE_W":                CORE_W,
        "boot_dS_proc_JK":      round(boot_dS, 8),
        "boot_dS_rate_JK_s":    round(boot_rate, 8),
        "mean_trial_dS_proc_JK": round(mean_dS, 8),
        "mean_trial_dS_rate_JK_s": round(trial_rate, 8),
        "rate_ratio_trial_vs_boot": round(rate_ratio, 4),
        "mean_comp_excess_JK":  round(mean_comp, 8),
        "mean_trial_dur_s":     round(mean_dur, 3),
        "mean_qemu_cpu_pct":    round(mean_qcpu, 2),
        "mean_landauer_JK":     round(mean_land, 10),
        "mean_est_instructions":int(mean_instr),
        "recovery_sys_util_ratio": round(rec_sys_ratio, 4),
    }
    results["analysis"] = analysis

    out_path = MOMONADOS_DIR / "delta_s_results.json"
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2)
    print(f"\n  Full results → {out_path}")
    return results


if __name__ == "__main__":
    run()
