#!/usr/bin/env python3
"""Every REPL command must be reachable from the menu.

The menu is what a user reads to find out what the kernel can do, and it had
drifted 33 commands behind the dispatch: sic, d12, d2048, triple, iuft, cycle,
weight, banked, trans, vessel, vita, spine, clay and more all worked and none
were listed. An agent typed `quantum_compile` — a real operation under another
name — and got "Unknown".

Aliases are exempt when the primary entry's description names them, so
`d2048 ... (alias d2k)` covers d2k without a line of its own.

    python3 check_menu_coverage.py     # exits 1 on any unlisted command
"""
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
repl = (HERE / "src" / "repl.rs").read_text(encoding="utf-8")
menu = (HERE / "src" / "menu.rs").read_text(encoding="utf-8")

arms = set()
for m in re.finditer(r'^            "([a-z0-9_\-]+)"((?:\s*\|\s*"[a-z0-9_\-?.]+")*)\s*=>',
                     repl, re.M):
    arms.add(m.group(1))
    arms |= set(re.findall(r'"([a-z0-9_\-?.]+)"', m.group(2) or ""))

listed = set(re.findall(r'cmd:\s*"([a-z0-9_\-]+)"', menu))
descs = " ".join(re.findall(r'desc:\s*"([^"]*)"', menu))

missing = sorted(c for c in arms - listed if c not in descs)
if missing:
    print(f"{len(missing)} REPL command(s) unreachable from the menu:")
    for c in missing:
        print(f"  {c}")
    print("\nAdd a MenuItem, or name it as an alias in the primary entry's desc.")
    sys.exit(1)

print(f"menu coverage OK — {len(arms)} REPL commands, "
      f"{len(listed)} menu entries, every command listed or named as an alias")
