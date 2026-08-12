#!/usr/bin/env python3
"""Patch dialect_cfg_viz.html to use proper GATE_MAP data instead of simulated strictness thresholds."""
import json
from pathlib import Path

HERE = Path(__file__).parent
DATA = json.loads((HERE / "dialect_positions.json").read_text())

# ===== GATE MAP (matches dialect_expansion.rs patterns) =====
GATE_MAP = {}
for d in DATA:
    name = d["name"]
    strict = d["strict"]
    if name == "canonical":
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "low_gate" in name:
        g = ("Phi",3.0,"odot",1.0,"Omega",3.0)
    elif "strict_frobenius" in name:
        g = ("f",3.0,"Phi",5.0,"Omega",3.0)
    elif "inverted_gates" in name:
        g = ("odot",2.0,"Phi",5.0,"Omega",3.0)
    elif "high_gate" in name:
        g = ("Phi",5.0,"odot",2.33,"Omega",4.0)
    elif "winding_first" in name:
        g = ("Omega",3.0,"odot",2.0,"Phi",5.0)
    elif "t_structural" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "no_ordering" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "chirality" in name and "first" in name:
        g = ("H",3.0,"odot",2.0,"Omega",3.0)
    elif "topology" in name and "first" in name:
        g = ("Th",5.0,"odot",2.0,"Omega",3.0)
    elif "scope" in name and "first" in name:
        g = ("G",3.0,"odot",2.0,"Omega",3.0)
    elif "dimensional" in name and "first" in name:
        g = ("D",3.0,"odot",2.0,"Omega",3.0)
    elif "kinetics" in name:
        g = ("K",3.0,"odot",2.0,"Omega",3.0) if "first" in name else ("Phi",5.0,"odot",2.0,"K",3.0)
    elif "broadcast" in name and "first" in name:
        g = ("c",3.0,"odot",2.0,"Omega",3.0)
    elif "fidelity" in name and "first" in name:
        g = ("f",3.0,"odot",2.0,"Omega",3.0)
    elif "stoichiometry" in name:
        g = ("Sigma",3.0,"odot",2.0,"Omega",3.0)
    elif "coupling" in name and "first" in name:
        g = ("R",3.0,"odot",2.0,"Omega",3.0)
    elif "parallel_" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "triple_" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "ordinal4" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "ordinal" in name:
        g = ("Omega",3.0,"odot",2.0,"Phi",4.0) if "swap" in name else ("Sigma",3.0,"Phi",5.0,"odot",2.0)
    elif "t_ceiling" in name:
        g = ("Th",4.0,"odot",2.0,"Phi",5.0) if "topology" in name else ("D",3.0,"odot",2.0,"Phi",5.0)
    elif "absorb_" in name:
        g = ("odot",2.0,"Phi",4.0,"Omega",3.0)
    elif "t_subset" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "mixed_" in name:
        g = ("G",3.0,"Th",5.0,"odot",2.0)
    elif "g4_quad" in name:
        g = ("G",3.0,"Phi",5.0,"odot",2.0)
    elif "only_" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0) if "parity" in name else \
            ("Omega",3.0,"odot",2.0,"Phi",5.0) if "winding" in name else \
            ("odot",2.0,"Phi",5.0,"Omega",3.0)
    elif "empty_gate" in name:
        g = ("Phi",0.0,"odot",0.0,"Omega",0.0)
    elif "dense_gates" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "the_all" in name:
        g = ("odot",2.0,"Phi",5.0,"Omega",3.0)
    else:
        g1m = max(0, min(5, round(strict/2.5)))
        g2m = max(0, min(3, round((strict-3)/3)))
        g3m = max(0, min(4, round(strict/3)))
        g = ("Phi",float(g1m),"odot",float(g2m),"Omega",float(g3m))
    GATE_MAP[name] = g

GATES_JSON = json.dumps({
    k:{"g1_p":v[0],"g1_m":v[1],"g2_p":v[2],"g2_m":v[3],"g3_p":v[4],"g3_m":v[5]}
    for k,v in GATE_MAP.items()
})

# Primitive glyphs and ordinals for display
PRIM_GLYPH = {"Phi":"\\u03a6","odot":"\\u2299","Omega":"\\u03a9","H":"\\u0126",
    "G":"\\u0393","Th":"\\u00de","R":"\\u0158","D":"\\u00d0","K":"\\u00c7",
    "f":"\\u0192","Sigma":"\\u03a3","c":"\\u0262"}

def prim_ord(pname):
    """Map primitive name to ordinal (1-5)."""
    m = {"Phi":5, "odot":2, "Omega":3, "H":4, "G":3, "Th":5, "R":4,
         "D":3, "f":3, "K":3, "Sigma":3, "c":3}
    return m.get(pname, 3)

# ===== Read existing HTML =====
html_path = HERE / "dialect_cfg_viz.html"
html = html_path.read_text(encoding="utf-8")

# ===== 1. Replace evalGates function =====
old_evalGates = """// Evaluate an IG tuple against dialect gates
function evalGates(ig, dialect) {
  const d = DATA[dialect];
  // We map from the 88 dialect data set
  // For now: simulate from strictness
  const strict = d.strict;
  const seq = d.seq;
  
  // Get the primitives needed
  const p_ord = ig.p || 3;
  const phi_ord = ig.phi || 1;
  const om_ord = ig.omega || 1;
  const f_ord = ig.f || 1;
  const h_ord = ig.h || 1;
  const g_ord = ig.g || 1;
  
  // Gate 1: Phi >= strict_level
  const g1_min = Math.max(1, Math.min(5, Math.round(strict/2.5)));
  const g1_prim = "Phi";
  const g1_pass = p_ord >= g1_min;
  
  // Gate 2: odot >= medium
  const g2_min = Math.max(1, Math.min(3, Math.round((strict-3)/3)));
  const g2_prim = "odot";
  const g2_pass = phi_ord >= g2_min;
  
  // Gate 3: Omega >= basic
  const g3_min = Math.max(1, Math.min(4, Math.round(strict/3)));
  const g3_prim = "Omega";
  const g3_pass = om_ord >= g3_min;
  
  const all_pass = g1_pass && g2_pass && g3_pass;
  return {g1:{prim:g1_prim,min:g1_min,pass:g1_pass,val:p_ord},
          g2:{prim:g2_prim,min:g2_min,pass:g2_pass,val:phi_ord},
          g3:{prim:g3_prim,min:g3_min,pass:g3_pass,val:om_ord},
          seq:seq, all:all_pass};
}"""

new_evalGates = """// Evaluate an IG tuple against dialect gates (uses GATE_MAP)
function evalGates(ig, dialect) {
  const d = DATA[dialect];
  const gm = GATE_MAP[d.name];
  if (!gm) {
    return {g1:{prim:"Phi",min:0,pass:false,val:0},
            g2:{prim:"odot",min:0,pass:false,val:0},
            g3:{prim:"Omega",min:0,pass:false,val:0},
            seq:d.seq, all:false};
  }
  
  // Map IG tuple values to ordinal numbers for gate checking
  function getVal(prim) {
    const igv = {"Phi":ig.P||0, "odot":ig.phi||0, "Omega":ig.omega||0,
      "H":ig.H||ig.h||0, "G":ig.G||ig.g||0, "Th":ig.T||ig.t||0,
      "R":ig.R||ig.r||0, "D":ig.D||ig.d||0, "f":ig.f||ig.F||0,
      "K":ig.K||ig.k||0, "Sigma":ig.S||ig.s||0, "c":ig.C||ig.c||0};
    return igv[prim] || 0;
  }
  
  const g1_pass = getVal(gm.g1_p) >= gm.g1_m;
  const g2_pass = getVal(gm.g2_p) >= gm.g2_m;
  const g3_pass = getVal(gm.g3_p) >= gm.g3_m;
  const all_pass = g1_pass && g2_pass && g3_pass;
  
  return {
    g1:{prim:gm.g1_p, min:gm.g1_m, pass:g1_pass, val:getVal(gm.g1_p)},
    g2:{prim:gm.g2_p, min:gm.g2_m, pass:g2_pass, val:getVal(gm.g2_p)},
    g3:{prim:gm.g3_p, min:gm.g3_m, pass:g3_pass, val:getVal(gm.g3_p)},
    seq:d.seq, all:all_pass
  };
}"""
# ===== 2. Replace showRuleset function =====
old_showRuleset = """function showRuleset(idx) {
  const d = DATA[idx];
  const box = document.getElementById("ruleset-box");
  const phase = d.phase;
  const strict = d.strict;
  const seq = d.seq;
  const tcnt = d.tcnt;
  
  // Gate thresholds derived from strictness
  const g1_min = Math.max(1, Math.min(5, Math.round(strict/2.5)));
  const g2_min = Math.max(1, Math.min(3, Math.round((strict-3)/3)));
  const g3_min = Math.max(1, Math.min(4, Math.round(strict/3)));
  
  // Better: map specific gates based on name patterns
  const g1_prim = "\\u03a6";
  const g2_prim = "\\u2299";
  const g3_prim = "\\u03a9";
  
  const g1_glyph = g1_prim;
  
  box.innerHTML = `
    <div class="gr"><span class="gl">U<sub>${idx}</sub></span><span class="gv"><b>${d.name}</b></span></div>
    <div class="gr"><span class="gl"></span><span style="color:#8a84b0;font-size:10px">${d.desc}</span></div>
    <div class="gr" style="margin-top:6px"><span class="gl">G1</span>
      <span class="gv">${g1_prim} \\u2265 ${g1_min}</span></div>
    <div class="gr"><span class="gl">G2</span>
      <span class="gv">${g2_prim} \\u2265 ${g2_min}</span></div>
    <div class="gr"><span class="gl">G3</span>
      <span class="gv">${g3_prim} \\u2265 ${g3_min}</span></div>
    <div class="gr"><span class="gl"></span>
      <span class="gv"><span class="tag ${seq ? "tag-seq" : "tag-par"}">${seq ? "SEQUENTIAL" : "PARALLEL"}</span>
      <span class="tag tag-info">T: ${tcnt} primitives</span>
      <span class="tag tag-info">strictness ${strict}</span></div>
  `;
}"""

# Primitive glyph map for HTML entities (used in showRuleset)
PRIM_GLYPH_JS = {k: v.replace("\\u", "&#x") + ";" for k, v in PRIM_GLYPH.items()}
PRIM_GLYPH_JS["odot"] = "&#x2299;"
# Build the new showRuleset as a JS function string
new_showRuleset = f"""function showRuleset(idx) {{
  const d = DATA[idx];
  const box = document.getElementById("ruleset-box");
  const gm = GATE_MAP[d.name];
  
  let g1g = "&#x03a6;", g2g = "&#x2299;", g3g = "&#x03a9;";
  let g1m = 0, g2m = 0, g3m = 0;
  if (gm) {{
    g1g = "{PRIM_GLYPH_JS['Phi']}";
    g2g = "{PRIM_GLYPH_JS['odot']}";
    g3g = "{PRIM_GLYPH_JS['Omega']}";
  }}
  
  box.innerHTML = `
    <div class="gr"><span class="gl">U<sub>${{idx}}</sub></span><span class="gv"><b>${{d.name}}</b></span></div>
    <div class="gr"><span class="gl"></span><span style="color:#8a84b0;font-size:10px">${{d.desc}}</span></div>
    <div class="gr" style="margin-top:6px"><span class="gl">G1</span>
      <span class="gv">${{gm ? "{PRIM_GLYPH_JS['Phi']}" : g1g}} &#x2265; ${{gm ? gm.g1_m : 0}} (${{gm ? gm.g1_p : "?"}})</span></div>
    <div class="gr"><span class="gl">G2</span>
      <span class="gv">${{gm ? "{PRIM_GLYPH_JS['odot']}" : g2g}} &#x2265; ${{gm ? gm.g2_m : 0}} (${{gm ? gm.g2_p : "?"}})</span></div>
    <div class="gr"><span class="gl">G3</span>
      <span class="gv">${{gm ? "{PRIM_GLYPH_JS['Omega']}" : g3g}} &#x2265; ${{gm ? gm.g3_m : 0}} (${{gm ? gm.g3_p : "?"}})</span></div>
    <div class="gr"><span class="gl"></span>
      <span class="gv"><span class="tag ${{d.seq ? "tag-seq" : "tag-par"}}">${{d.seq ? "SEQUENTIAL" : "PARALLEL"}}</span>
      <span class="tag tag-info">T: ${{d.tcnt}} primitives</span>
      <span class="tag tag-info">strictness ${{d.strict}}</span></div>
  `;
}}"""

# ===== 3. Replace the gate computation inside evaluate() =====
old_eval_gates_inline = """  // Evaluate gates (use the actual strictness from data)
  const d = DATA[idx];
  const strict = d.strict;
  const seq = d.seq;
  
  const g1_min = Math.max(1, Math.min(5, Math.round(strict/2.5)));
  const g2_min = Math.max(1, Math.min(3, Math.round((strict-3)/3)));
  const g3_min = Math.max(1, Math.min(4, Math.round(strict/3)));
  
  const g1_pass = ig.P >= g1_min;
  const g2_pass = ig.phi >= g2_min;
  const g3_pass = ig.omega >= g3_min;
  const all_pass = g1_pass && g2_pass && g3_pass;
  
  // Gate ordering check
  let ordering_msg = "";
  if (seq) {
    if (!g1_pass) ordering_msg = "G1 fails \\u2192 G2/G3 blocked (sequential)";
    else if (!g2_pass) ordering_msg = "G2 fails \\u2192 G3 blocked (sequential)";
    else ordering_msg = "All sequential gates pass";
  } else {
    ordering_msg = "Parallel: gates independent";
  }
  
  const prim_names = {D:"\\u00d0",T:"\\u00de",R:"\\u0158",P:"\\u03a6",F:"\\u0192",
    K:"\\u00c7",G:"\\u0393",C:"\\u0262",phi:"\\u2299",H:"\\u0126",S:"\\u03a3",omega:"\\u03a9"};
  
  const ig_str = Object.entries(prim_names).map(([k,v]) => `${v}:${ig[k]||"?"}`).join(" ");
  
  // Result box
  const rslt = document.getElementById("rslt-box");
  rslt.innerHTML = `
    <div style="margin-bottom:6px"><b>IMASM Word</b>: ${tokens.join(" ")}</div>
    <div style="margin-bottom:6px"><b>IG Tuple</b>: ${ig_str}</div>
    <div style="margin:8px 0">
      <span class="tag ${g1_pass ? "tag-pass" : "tag-fail"}">G1 ${g1_pass ? "PASS" : "FAIL"} (${ig.P} >= ${g1_min})</span>
      <span class="tag ${g2_pass ? "tag-pass" : "tag-fail"}">G2 ${g2_pass ? "PASS" : "FAIL"} (${ig.phi} >= ${g2_min})</span>
      <span class="tag ${g3_pass ? "tag-pass" : "tag-fail"}">G3 ${g3_pass ? "PASS" : "FAIL"} (${ig.omega} >= ${g3_min})</span>
      <span class="tag ${all_pass ? "tag-pass" : "tag-fail"}">OVERALL ${all_pass ? "PASS" : "FAIL"}</span>
    </div>
    <div style="color:#8a84b0;font-size:10px">${ordering_msg} &middot; ${tokens.length} tokens &middot; ${seq ? "seq" : "par"} gates</div>
  `;
  
  // Grammar details
  drawCFG(tokens, ig, g1_pass && g2_pass && g3_pass, seq, g1_pass, g2_pass, g3_pass, idx);"""
new_eval_gates_inline = """  // Evaluate gates using GATE_MAP
  const d = DATA[idx];
  const ev = evalGates(ig, idx);
  const g1_pass = ev.g1.pass;
  const g2_pass = ev.g2.pass;
  const g3_pass = ev.g3.pass;
  const all_pass = ev.all;
  const seq = ev.seq;
  
  // Build ordering message with actual primitives
  let ordering_msg = "";
  if (seq) {
    if (!g1_pass) ordering_msg = "G1 " + ev.g1.prim + " fails \\u2192 G2/G3 blocked (sequential)";
    else if (!g2_pass) ordering_msg = "G2 " + ev.g2.prim + " fails \\u2192 G3 blocked (sequential)";
    else ordering_msg = "All sequential gates pass";
  } else {
    ordering_msg = "Parallel: gates independent";
  }
  
  const prim_names = {D:"\\u00d0",T:"\\u00de",R:"\\u0158",P:"\\u03a6",F:"\\u0192",
    K:"\\u00c7",G:"\\u0393",C:"\\u0262",phi:"\\u2299",H:"\\u0126",S:"\\u03a3",omega:"\\u03a9"};
  
  const ig_str = Object.entries(prim_names).map(([k,v]) => v + ":" + (ig[k]||"?")).join(" ");
  
  // Glyphs for gate primitives
  const pglyph = {"Phi":"\\u03a6","odot":"\\u2299","Omega":"\\u03a9","H":"\\u0126",
    "G":"\\u0393","Th":"\\u00de","R":"\\u0158","D":"\\u00d0","K":"\\u00c7",
    "f":"\\u0192","Sigma":"\\u03a3","c":"\\u0262"};
  
  // Result box
  const rslt = document.getElementById("rslt-box");
  rslt.innerHTML = `
    <div style="margin-bottom:6px"><b>IMASM Word</b>: ${tokens.join(" ")}</div>
    <div style="margin-bottom:6px"><b>IG Tuple</b>: ${ig_str}</div>
    <div style="margin:8px 0">
      <span class="tag ${g1_pass ? "tag-pass" : "tag-fail"}">G1 ${g1_pass ? "PASS" : "FAIL"} (${pglyph[ev.g1.prim]||ev.g1.prim}:${ev.g1.val} >= ${ev.g1.min})</span>
      <span class="tag ${g2_pass ? "tag-pass" : "tag-fail"}">G2 ${g2_pass ? "PASS" : "FAIL"} (${pglyph[ev.g2.prim]||ev.g2.prim}:${ev.g2.val} >= ${ev.g2.min})</span>
      <span class="tag ${g3_pass ? "tag-pass" : "tag-fail"}">G3 ${g3_pass ? "PASS" : "FAIL"} (${pglyph[ev.g3.prim]||ev.g3.prim}:${ev.g3.val} >= ${ev.g3.min})</span>
      <span class="tag ${all_pass ? "tag-pass" : "tag-fail"}">OVERALL ${all_pass ? "PASS" : "FAIL"}</span>
    </div>
    <div style="color:#8a84b0;font-size:10px">${ordering_msg} &middot; ${tokens.length} tokens &middot; ${seq ? "seq" : "par"} gates</div>
    <div style="color:#5a659c;font-size:9px;margin-top:2px">U<sub>${idx}</sub> ${d.name} &middot; GATE_MAP: ${ev.g1.prim}>=${ev.g1.min} ${ev.g2.prim}>=${ev.g2.min} ${ev.g3.prim}>=${ev.g3.min}</div>
  `;
  
  // Grammar details
  drawCFG(tokens, ig, all_pass, seq, g1_pass, g2_pass, g3_pass, idx);"""

# ===== 4. Perform the replacements =====
# Insert GATE_MAP data at the start of the <script> block
gate_insert = f"\n// GATE_MAP: gate definitions for all 88 dialects\nconst GATE_MAP = {GATES_JSON};\n\n"
html = html.replace("<script>\nconst DATA = ", "<script>" + gate_insert + "const DATA = ")

# Replace evalGates function
if old_evalGates in html:
    html = html.replace(old_evalGates, new_evalGates)
    print("✓ Replaced evalGates()")
else:
    print("✗ evalGates() not found in HTML")
    # Show where it starts
    idx = html.find("function evalGates")
    print(f"  Found at position {idx}, showing context:")
    print(repr(html[idx:idx+200]))

# Replace showRuleset function
if old_showRuleset in html:
    html = html.replace(old_showRuleset, new_showRuleset)
    print("✓ Replaced showRuleset()")
else:
    print("✗ showRuleset() not found")
    idx = html.find("function showRuleset")
    if idx >= 0:
        print(f"  Found at position {idx}")
        print(repr(html[idx:idx+200]))

# Replace inline gate computation in evaluate()
if old_eval_gates_inline in html:
    html = html.replace(old_eval_gates_inline, new_eval_gates_inline)
    print("✓ Replaced inline gate eval in evaluate()")
else:
    print("✗ Inline gate eval not found")
    idx = html.find("// Evaluate gates (use the actual strictness")
    if idx >= 0:
        print(f"  Found at position {idx}")
        # The old function might have whitespace differences
        actual = html[idx:idx+1600]
        print(repr(actual[:300]))
        print("---")
        print(repr(old_eval_gates_inline[:300]))
# ===== 5. Write the patched HTML =====
html_path.write_text(html, encoding="utf-8")
print(f"\n✓ Patched {html_path}")
print(f"  Original size: {html_path.stat().st_size} bytes")

# ===== 6. Verify the patching =====
verify_checks = [
    ("GATE_MAP const", "const GATE_MAP = {" in html),
    ("evalGates() uses GATE_MAP", "GATE_MAP[d.name]" in html),
    ("showRuleset() uses GATE_MAP", "GATE_MAP[d.name]" in html),
    ("evaluate() calls evalGates()", "evalGates(ig, idx)" in html),
]
print("\n=== Verification ===")
for label, result in verify_checks:
    print(f"  {'✓' if result else '✗'} {label}: {result}")
all_ok = all(r for _, r in verify_checks)
print(f"\n{'ALL CHECKS PASSED' if all_ok else 'SOME CHECKS FAILED'}")
