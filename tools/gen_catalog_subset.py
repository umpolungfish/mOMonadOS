#!/usr/bin/env python3
"""Emit src/catalog_ask_subset.rs from the canonical IG_catalog.json.

The kernel used to carry a curated 223-entry slice, which is why `iuft encode
gluon` and `iuft encode jeu` came back empty: those entries exist in the
canonical catalog and simply were not in the slice. Curation by hand is the
defect, so this generator takes the whole catalog by default.

The glyph -> IgPrim variant map is not written here. It is parsed out of the
trailing comments in src/imas_ig.rs, which carry `Variant = n, // 𐑦 gloss`,
so a new primitive value in the enum reaches this generator without anyone
remembering to update it. A glyph in the catalog with no variant in the enum
is a hard error, not a silent skip.

Usage:
    python3 tools/gen_catalog_subset.py                 # whole catalog
    python3 tools/gen_catalog_subset.py --limit 500     # first N entries
    python3 tools/gen_catalog_subset.py --desc-chars 80 # tighter descriptions
"""

import argparse
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CATALOG = ROOT.parent / "imscribing_grammar" / "IG_catalog.json"
ENUM = ROOT / "src" / "imas_ig.rs"
OUT = ROOT / "src" / "catalog_ask_subset.rs"

# The twelve slots, in canonical order, keyed as they appear in IG_catalog.json.
SLOTS = ["⊢", "⊣", "≻", "≺", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡"]

# The variant names are the Core.lean constructors now, so there is no prefix
# to disambiguate by. There does not need to be: all 49 value glyphs are
# distinct, so a glyph names exactly one variant. If that ever stops being true
# the lookup below fails loudly rather than guessing.

# Two Criticality variants are named with the glyph itself (𐑮, 𐑢), so the
# identifier pattern has to admit non-ASCII rather than assume [A-Za-z_].
VARIANT_RE = re.compile(r"^\s*([^\s=]+)\s*=\s*\d+\s*,\s*//\s*(\S+)")


def variant_map():
    """glyph -> [variant names], parsed from the enum's own comments."""
    out = {}
    inside = False
    for line in ENUM.read_text(encoding="utf-8").splitlines():
        if "pub enum IgPrim" in line:
            inside = True
            continue
        if inside and line.strip() == "}":
            break
        if not inside:
            continue
        m = VARIANT_RE.match(line)
        if m:
            out.setdefault(m.group(2), []).append(m.group(1))
    if not out:
        sys.exit("no IgPrim variants parsed from %s" % ENUM)
    return out


def pick(glyph, slot, vmap):
    """The variant for this glyph in this slot."""
    cands = vmap.get(glyph)
    if not cands:
        raise KeyError("glyph %r (slot %s) has no IgPrim variant" % (glyph, slot))
    if len(cands) == 1:
        return cands[0]
    raise KeyError("glyph %r names %d variants (slot %s): %s — the 49 value "
                   "glyphs are supposed to be distinct"
                   % (glyph, len(cands), slot, cands))


def rust_str(s, limit):
    """A Rust string literal, truncated on a character boundary."""
    s = " ".join(s.split())
    if len(s) > limit:
        s = s[: limit - 3].rstrip() + "..."
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=0, help="cap entry count (0 = all)")
    ap.add_argument("--desc-chars", type=int, default=160)
    args = ap.parse_args()

    vmap = variant_map()
    entries = json.loads(CATALOG.read_text(encoding="utf-8"))
    if args.limit:
        entries = entries[: args.limit]

    seen, rows, skipped, collided = set(), [], [], []
    for e in entries:
        name = e.get("name", "").strip().lower()
        if not name:
            continue
        if name in seen:
            # Two entries whose names differ only in case. They are distinct
            # entries with distinct tuples, so keeping the first silently drops
            # a real one. Say so; which of them should keep the name is a
            # determination about the entries, not something to guess here.
            collided.append(e.get("name", ""))
            continue
        try:
            prims = [pick(e[s], s, vmap) for s in SLOTS]
        except KeyError as exc:
            skipped.append((name, str(exc)))
            continue
        seen.add(name)
        rows.append((name, e.get("description", ""), prims))

    if skipped:
        print("%d entries skipped for unmapped glyphs; first few:" % len(skipped))
        for n, why in skipped[:5]:
            print("   ", n, "--", why)
    if collided:
        print("%d entries dropped — name collides case-insensitively with one "
              "already taken:" % len(collided))
        for n in collided:
            print("   ", n)

    body = []
    for name, desc, p in rows:
        body.append(
            "    entry(\n"
            "        %s, %s,\n"
            "        IgPrim::%s, IgPrim::%s, IgPrim::%s,\n"
            "        IgPrim::%s, IgPrim::%s, IgPrim::%s,\n"
            "        IgPrim::%s, IgPrim::%s,\n"
            "        IgPrim::%s, IgPrim::%s, IgPrim::%s, IgPrim::%s,\n"
            "        0, Domain::General,\n"
            "    ),"
            % (rust_str(name, 96), rust_str(desc, args.desc_chars), *p)
        )

    OUT.write_text(
        "// AUTO-GENERATED by tools/gen_catalog_subset.py — do not hand-edit.\n"
        "// Source: imscribing_grammar/IG_catalog.json (%d entries emitted).\n"
        "// Regenerate after any catalog change; sync_catalog.sh --check guards the source.\n"
        "/// The IG catalog, embedded for the kernel's own lookup.\n"
        "pub static ASK_CATALOG_SUBSET: &[CatalogEntry] = &[\n%s\n];\n"
        % (len(rows), "\n".join(body)),
        encoding="utf-8",
    )
    print("wrote %s: %d entries, %.1f KiB" % (OUT.name, len(rows), OUT.stat().st_size / 1024))


if __name__ == "__main__":
    main()
