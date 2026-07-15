#!/usr/bin/env python3
"""
generate_rust_catalog.py — Generate Rust CatalogEntry definitions from IG_catalog.json

Reads the full IG catalog JSON and outputs Rust const entry() definitions
that can be inserted into mOMonadOS/src/catalog.rs.

Usage:
  python3 generate_rust_catalog.py [--limit N] [--domain DOMAIN]

Output goes to stdout. Pipe to a file or redirect to add to catalog.rs.
"""

import json
import sys
import os

# Shavian glyph → Rust IgPrim variant mapping
SHavian_TO_RUST = {
    # D (Dimensionality)
    "𐑦": "IgPrim::D_odot",
    "𐑛": "IgPrim::D_wedge",
    "𐑨": "IgPrim::D_triangle",
    "𐑼": "IgPrim::D_infty",
    # T (Topology)
    "𐑡": "IgPrim::T_net",
    "𐑰": "IgPrim::T_in",
    "𐑥": "IgPrim::T_bowtie",
    "𐑶": "IgPrim::T_boxtimes",
    "𐑸": "IgPrim::T_odot",
    # R (Coupling)
    "𐑩": "IgPrim::R_super",
    "𐑑": "IgPrim::R_cat",
    "𐑽": "IgPrim::R_dagger",
    "𐑾": "IgPrim::R_lr",
    # P (Parity/Symmetry)
    "𐑗": "IgPrim::P_asym",
    "𐑿": "IgPrim::P_psi",
    "𐑬": "IgPrim::P_pm",
    "𐑯": "IgPrim::P_sym",
    "𐑹": "IgPrim::P_pmsym",
    # F (Fidelity)
    "𐑱": "IgPrim::F_ell",
    "𐑞": "IgPrim::F_eth",
    "𐑐": "IgPrim::F_hbar",
    # K (Kinetics)
    "𐑘": "IgPrim::K_fast",
    "𐑤": "IgPrim::K_mod",
    "𐑧": "IgPrim::K_slow",
    "𐑪": "IgPrim::K_trap",
    "𐑺": "IgPrim::K_mbl",
    # G (Range/Cardinality)
    "𐑚": "IgPrim::G_beth",
    "𐑔": "IgPrim::G_gimel",
    "𐑲": "IgPrim::G_aleph",
    # C/ɢ (Composition)
    "𐑝": "IgPrim::C_and",
    "𐑜": "IgPrim::C_or",
    "𐑠": "IgPrim::C_seq",
    "𐑵": "IgPrim::C_broad",
    # φ̂ (Criticality)
    "𐑢": "IgPrim::𐑢",
    "⊙": "IgPrim::⊙",
    "𐑮": "IgPrim::𐑮",
    "𐑻": "IgPrim::𐑻",
    "𐑣": "IgPrim::Phi_super",
    # H (Chirality)
    "𐑓": "IgPrim::H0",
    "𐑒": "IgPrim::H1",
    "𐑖": "IgPrim::H2",
    "𐑫": "IgPrim::H_inf",
    # S (Stoichiometry)
    "𐑙": "IgPrim::S_11",
    "𐑕": "IgPrim::S_many",
    "𐑳": "IgPrim::S_nm",
    # Ω (Winding)
    "𐑷": "IgPrim::Omega_0",
    "𐑴": "IgPrim::Omega_z2",
    "𐑭": "IgPrim::Omega_z",
    "𐑟": "IgPrim::Omega_na",
}

# Domain mapping
DOMAIN_MAP = {
    "Mathematics": "Domain::Mathematics",
    "Physics": "Domain::Physics",
    "Biology": "Domain::Biology",
    "Consciousness": "Domain::Consciousness",
    "Language": "Domain::Language",
    "Civilization": "Domain::Civilization",
    "Ecology": "Domain::Ecology",
    "General": "Domain::General",
}

# Tier → numeric
TIER_MAP = {"O₀": 0, "O₁": 1, "O₂": 2, "O₂†": 3, "O_∞": 4, "O_∞⁺": 5}

def find_catalog_path():
    candidates = [
        os.path.join(os.path.dirname(__file__), "..", "imscribe.com", "IG_catalog.json"),
        os.path.join(os.path.dirname(__file__), "..", "red-hot_rebis", "shared", "IG_catalog.json"),
        "/home/mrnob0dy666/imsgct/imscribe.com/IG_catalog.json",
    ]
    for p in candidates:
        if os.path.exists(p):
            return os.path.abspath(p)
    return None

def sanitize_rust_ident(name):
    """Convert a catalog name to a Rust const identifier."""
    return name.replace("-", "_").replace(" ", "_").replace(".", "_").upper()

def escape_rust_str(s):
    """Escape a string for use in Rust source."""
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', ' ')

def main():
    import argparse
    parser = argparse.ArgumentParser(description="Generate Rust CatalogEntry from IG_catalog.json")
    parser.add_argument("--limit", type=int, default=0, help="Limit entries (0=all)")
    parser.add_argument("--domain", type=str, default=None, help="Filter by domain")
    parser.add_argument("--skip-existing", action="store_true", help="Skip entries already in static catalog")
    args = parser.parse_args()

    path = find_catalog_path()
    if not path:
        print("// ERROR: IG_catalog.json not found", file=sys.stderr)
        sys.exit(1)

    with open(path) as f:
        raw = json.load(f)

    if isinstance(raw, list):
        entries = raw
    elif isinstance(raw, dict) and "imscriptions" in raw:
        entries = raw["imscriptions"]
    else:
        entries = list(raw.values()) if isinstance(raw, dict) else []

    # Already-in-catalog names (from Rust static catalog)
    existing = {
        "zfc", "zfc_t", "zfc_fe", "clink_l8",
        "temporal_mathematics", "schrodinger", "heat_diffusion",
        "navier_stokes", "wave_equation", "einstein",
        "universal_imscriptive_grammar", "o_inf", "o_0", "yhwh",
    }

    count = 0
    print("// Auto-generated from IG_catalog.json — add to STATIC_CATALOG array")
    print(f"// Generated: {len(entries)} total entries in catalog")
    print()

    for entry in entries:
        name = entry.get("name", "")
        if not name:
            continue
        if args.skip_existing and name in existing:
            continue
        if args.domain:
            domain = entry.get("domain", "General")
            if domain != args.domain:
                continue

        ident = sanitize_rust_ident(name)
        desc = escape_rust_str(entry.get("description", ""))
        if len(desc) > 200:
            desc = desc[:197] + "..."

        # Map primitives
        prims = {}
        for key, rust_key in [("Ð","d"), ("Þ","t"), ("Ř","r"), ("Φ","p"),
                               ("ƒ","f"), ("Ç","k"), ("Γ","g"), ("ɢ","c"),
                               ("⊙","phi"), ("Ħ","h"), ("Σ","s"), ("Ω","omega")]:
            glyph = entry.get(key, "")
            rust_variant = SHavian_TO_RUST.get(glyph)
            if rust_variant:
                prims[rust_key] = rust_variant
            else:
                prims[rust_key] = f"// UNKNOWN: {key}={glyph}"

        tier_str = entry.get("tier", "O₀")
        tier = TIER_MAP.get(tier_str, 0)
        domain_str = entry.get("domain", "General")
        domain_rust = DOMAIN_MAP.get(domain_str, "Domain::General")

        # Skip entries where we couldn't map all primitives
        if any(v.startswith("// UNKNOWN") for v in prims.values()):
            continue

        print(f"const {ident}: CatalogEntry = entry(")
        print(f'    "{name}", "{desc}",')
        print(f"    {prims['d']}, {prims['t']}, {prims['r']},")
        print(f"    {prims['p']}, {prims['f']}, {prims['k']},")
        print(f"    {prims['g']}, {prims['c']},")
        print(f"    {prims['phi']}, {prims['h']}, {prims['s']}, {prims['omega']},")
        print(f"    {tier}, {domain_rust},")
        print(f");")
        print()

        count += 1
        if args.limit and count >= args.limit:
            break

    print(f"// {count} entries generated")
    print(f"// Add to STATIC_CATALOG: " + ", ".join([sanitize_rust_ident(e.get("name","")) for e in entries[:5] if e.get("name")]) + ", ...")

if __name__ == "__main__":
    main()
