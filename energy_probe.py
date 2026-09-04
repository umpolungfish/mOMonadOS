#!/usr/bin/env python3
"""Meter package energy for a chain of factorization splits against an idle
control, reading Intel RAPL directly from the package energy MSR.

Must run as root: reading /dev/cpu/0/msr needs it, and the msr module is loaded
here if it is not already. On WSL2 the read may fault or stay flat if the
hypervisor does not pass the MSR through; the script says so plainly rather than
reporting a fabricated number.

  sudo python3 energy_probe.py [N]

N is the number of splits in the active chain (default 160). The split is the
store-free leaping search on a semiprime whose loop length is above 4e10.
"""
import os, sys, time, struct, subprocess

MSR_RAPL_POWER_UNIT = 0x606
MSR_PKG_ENERGY_STATUS = 0x611
HERE = os.path.dirname(os.path.abspath(__file__))
SPLIT = "trilattice_factor winding 1000036000099"

def read_msr(reg, cpu=0):
    path = f"/dev/cpu/{cpu}/msr"
    fd = os.open(path, os.O_RDONLY)
    try:
        os.lseek(fd, reg, os.SEEK_SET)
        return struct.unpack("<Q", os.read(fd, 8))[0]
    finally:
        os.close(fd)

def ensure_msr():
    if not os.path.exists("/dev/cpu/0/msr"):
        subprocess.run(["modprobe", "msr"], check=False)
    if not os.path.exists("/dev/cpu/0/msr"):
        sys.exit("no /dev/cpu/0/msr after modprobe msr; MSR not available in this guest")

def energy_joules(esu_div):
    raw = read_msr(MSR_PKG_ENERGY_STATUS) & 0xFFFFFFFF
    return raw / esu_div, raw

def delta_j(before_raw, after_raw, esu_div):
    d = (after_raw - before_raw) & 0xFFFFFFFF  # 32-bit wrap
    return d / esu_div

def run_chain(n):
    args = [os.path.join(HERE, "run_hosted_cmds.sh")] + [SPLIT] * n
    subprocess.run(args, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=False)

def main():
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 160
    ensure_msr()
    unit = read_msr(MSR_RAPL_POWER_UNIT)
    esu = (unit >> 8) & 0x1F
    esu_div = float(1 << esu)  # counts per joule
    print(f"RAPL energy unit: 2^-{esu} J per count")

    # sanity: does the counter move at all over a short busy spin?
    _, r0 = energy_joules(esu_div)
    t = time.time()
    while time.time() - t < 0.5:
        pass
    _, r1 = energy_joules(esu_div)
    if ((r1 - r0) & 0xFFFFFFFF) == 0:
        sys.exit("package energy counter did not advance; the guest is not passing RAPL through")

    # idle control: same wall time the chain will take, measured first at rest.
    _, ia = energy_joules(esu_div)
    t0 = time.time()
    run_chain(n)
    chain_wall = time.time() - t0
    _, ib = energy_joules(esu_div)
    active_j = delta_j(ia, ib, esu_div)

    _, ja = energy_joules(esu_div)
    time.sleep(chain_wall)
    _, jb = energy_joules(esu_div)
    idle_j = delta_j(ja, jb, esu_div)

    print(f"chain of {n} splits: wall {chain_wall:.2f}s")
    print(f"  package energy over the chain : {active_j:.2f} J")
    print(f"  package energy idle, same wall: {idle_j:.2f} J")
    marg = active_j - idle_j
    print(f"  marginal energy of the splits : {marg:.2f} J  ({marg/n*1000:.2f} mJ per split)")
    print(f"  idle baseline power           : {idle_j/chain_wall:.2f} W")
    print(f"  active power                  : {active_j/chain_wall:.2f} W")

if __name__ == "__main__":
    main()
