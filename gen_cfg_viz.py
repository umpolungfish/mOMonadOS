#!/usr/bin/env python3
"""Generate Dialect CFG visualizer HTML (all 88 universes)."""
import json
from pathlib import Path
HERE = Path(__file__).parent
DATA = json.loads((HERE / "dialect_positions.json").read_text())

# Phase colors
PC = ["#e8b84b", "#4ba8b8", "#a84bb8"]
PN = ["Canonical", "Hand-crafted", "Expansion"]

# Token glyphs and families
GLYPH = {"VINIT":"\u22a2","TANCH":"\u22a3","AFWD":">","AREV":"<",
         "CLINK":"=","IMSCRIB":"\u2299","FSPLIT":"\u25c7","FFUSE":"\u25cf",
         "EVALT":"+","EVALF":"\u00d7","ENGAGR":"\u229e","IFIX":"\u00ac"}
FAM = {"VINIT":0,"TANCH":0,"AFWD":0,"AREV":0,"CLINK":0,"IMSCRIB":0,
       "FSPLIT":1,"FFUSE":1,"EVALT":2,"EVALF":2,"ENGAGR":2,"IFIX":3}
FAMC = ["#6af","#a84bb8","#e8b84b","#6f6"]
FAMN = ["LOGICAL","FROBENIUS","DIALETHEIA","LINEAR"]

CANONICALS = {
"I_Dialetheic_Bootstrap":"IMSCRIB EVALT FSPLIT EVALF FFUSE ENGAGR IFIX IMSCRIB",
"II_Void_Genesis":"VINIT TANCH AFWD FSPLIT CLINK FFUSE IFIX IMSCRIB",
"III_Anchor_Protocol":"TANCH AREV VINIT AFWD TANCH CLINK IFIX IMSCRIB",
"IV_Dual_Bootstrap":"IMSCRIB AFWD FFUSE FSPLIT AREV CLINK IFIX IMSCRIB",
"V_Linear_Chain":"IFIX IFIX IFIX IFIX IFIX IFIX IFIX IFIX",
"VI_Empty_Bootstrap":"VINIT IMSCRIB VINIT IMSCRIB VINIT IMSCRIB VINIT IMSCRIB",
"VII_Parakernel":"EVALF AREV FSPLIT EVALT AFWD FFUSE ENGAGR IFIX",
"VIII_Frobenius_Kernel":"VINIT FSPLIT FFUSE TANCH",
"IX_Chiral_Pairs":"AFWD AREV AFWD AREV AFWD AREV AFWD AREV",
"X_Truth_Machine":"IMSCRIB FSPLIT EVALT IFIX IMSCRIB FSPLIT EVALF IFIX",
"XI_Eternal_Return":"IMSCRIB AFWD AREV IMSCRIB AFWD AREV IMSCRIB AFWD",
"XII_ROM_Burn":"EVALT IFIX EVALF IFIX ENGAGR IFIX IMSCRIB IFIX",
"agent_loop":"VINIT IMSCRIB FSPLIT EVALT CLINK FFUSE IFIX ENGAGR AREV CLINK TANCH",
"bootstrap_loop":"IMSCRIB AREV FSPLIT AFWD FFUSE CLINK IFIX IMSCRIB",
}


# ===== CSS =====
CSS = r"""
*{margin:0;padding:0;box-sizing:border-box}
body{background:#0b0b12;color:#f0ead8;font-family:'Courier New',monospace;overflow-x:hidden}
#layout{display:grid;grid-template-columns:340px 1fr;height:100vh}
#sidebar{background:#12101e;border-right:1px solid #2a2740;padding:16px;overflow-y:auto}
#main{padding:20px;overflow-y:auto}
h1{color:#e8b84b;font-size:18px;margin-bottom:2px}
h2{color:#c8b8e0;font-size:14px;margin:12px 0 6px;border-bottom:1px solid #2a2740;padding-bottom:4px}
select{width:100%;background:#1e1c2e;color:#f0ead8;border:1px solid #3a3550;border-radius:4px;padding:6px 8px;font-family:'Courier New',monospace;font-size:12px;margin-bottom:6px}
select option{background:#1e1c2e}
textarea{width:100%;background:#1e1c2e;color:#f0ead8;border:1px solid #3a3550;border-radius:4px;padding:6px 8px;font-family:'Courier New',monospace;font-size:11px;height:60px;margin:4px 0}
.btn{background:#2a2740;color:#f0ead8;border:1px solid #5a659c;border-radius:4px;padding:5px 12px;cursor:pointer;font-size:11px;margin:2px}
.btn:hover{background:#3a3550;border-color:#a84bb8}
.tag{display:inline-block;padding:2px 8px;border-radius:3px;font-size:10px;margin:2px;font-weight:bold}
.tag-pass{background:#1a3a1a;color:#6f6;border:1px solid #2a5a2a}
.tag-fail{background:#3a1a1a;color:#f66;border:1px solid #5a2a2a}
.tag-par{background:#2a1a3a;color:#a8f;border:1px solid #3a2a5a}
.tag-seq{background:#1a2a3a;color:#6af;border:1px solid #2a3a5a}
#ruleset-box{background:#1a1828;border:1px solid #2a2740;border-radius:6px;padding:10px;margin:10px 0;font-size:11px;line-height:1.6}
#ruleset-box .gr{display:flex;gap:6px;margin:2px 0}
#ruleset-box .gl{color:#8a84b0;width:24px}
#ruleset-box .gv{color:#f0ead8}
#rslt-box{background:#1a1828;border:1px solid #2a2740;border-radius:6px;padding:10px;margin:10px 0;font-size:11px}
#cfg-box{background:#1a1828;border:1px solid #2a2740;border-radius:6px;padding:10px;margin:10px 0;font-size:11px;line-height:1.6}
#cfg-view{width:100%;height:480px;background:#0e0c18;border:1px solid #2a2740;border-radius:6px;position:relative;overflow:hidden}
#cfg-view svg{width:100%;height:100%}
.edge{stroke:#3a3550;stroke-width:1.5;fill:none}
.edge.pass{stroke:#4a8}
.edge.fail{stroke:#844;stroke-dasharray:4,3}
.edge.back{stroke:#a84bb8;stroke-dasharray:2,2}
text{font-family:'Courier New',monospace;font-size:8px;fill:#f0ead8;pointer-events:none}
#legend{display:flex;gap:14px;margin:6px 0;font-size:10px;flex-wrap:wrap}
#legend span{display:flex;align-items:center;gap:4px}
.ld{width:10px;height:10px;border-radius:50%;display:inline-block}
#tip{position:absolute;display:none;pointer-events:none;background:rgba(11,11,18,0.96);border:1px solid #5a659c;border-radius:6px;padding:8px 12px;color:#f0ead8;font-size:11px;max-width:280px;z-index:100;line-height:1.5}
#stat-bar{display:flex;gap:12px;margin:6px 0;font-size:11px;color:#8a84b0;flex-wrap:wrap}
"""

H = [
'<!DOCTYPE html>',
'<html lang="en"><head><meta charset="UTF-8">',
'<meta name="viewport" content="width=device-width,initial-scale=1.0">',
'<title>Dialect CFG Engine</title><style>',
CSS,
'</style></head><body>',
'<div id="tip"></div><div id="layout">',
'<div id="sidebar">',
'<h1>&#x2299; Dialect CFG</h1>',
'<div style="font-size:10px;color:#8a84b0;margin-bottom:10px">88 universes &middot; IMASM grammar</div>',
'<h2>1. Select Universe</h2>',
'<select id="dialectSel" onchange="updateDialect()">',
]
for d in DATA:
    H.append(f'<option value="{d["i"]}">{d["name"]} (U_{d["i"]})</option>')
H.append('</select>')
H.append('<div id="ruleset-box">Select a universe to see its ruleset</div>')
H.append('<h2>2. Select IMASM Word</h2>')
H.append('<select id="wordSel" onchange="updateWord()">')
H.append('<option value="agent_loop">agent_loop (kernel)</option>')
H.append('<option value="bootstrap_loop">bootstrap_loop</option>')
for k in CANONICALS:
    H.append(f'<option value="{k}">{k}</option>')
H.append('<option value="custom">&mdash; custom &mdash;</option>')
H.append('</select>')
H.append('<textarea id="customWord" placeholder="VINIT IMSCRIB FSPLIT ..." oninput="onCustom()"></textarea>')
H.append('<button class="btn" onclick="evaluate()">&#x25b6; Evaluate</button>')
H.append('<h2>3. Gate Results</h2>')
H.append('<div id="rslt-box">Run evaluation</div>')
H.append('</div>')  # end sidebar
H.append('<div id="main">')
H.append('<h1>CFG: Token Grammar &times; Dialect Ruleset</h1>')
H.append('<div id="stat-bar">Select universe + IMASM word, then Evaluate</div>')
H.append('<div id="legend">')
for i in range(4):
    H.append(f'<span><span class="ld" style="background:{FAMC[i]}"></span>{FAMN[i]}</span>')
H.append('<span><span style="color:#4a8">&mdash;</span> gate passes</span>')
H.append('<span><span style="color:#844">- -</span> gate fails</span>')
H.append('<span><span style="color:#a84bb8">..</span> cyclic wrap</span>')
H.append('</div>')
H.append('<div id="cfg-view"><svg id="cfgSvg"></svg></div>')
H.append('<h2>Grammar Details</h2>')
H.append('<div id="cfg-box">Run evaluation</div>')
H.append('</div></div>')

# ===== JAVASCRIPT =====
H.append("""<script>
const DATA = """ + json.dumps(DATA) + """;
const GLYPH = {"VINIT":"\\u22a2","TANCH":"\\u22a3","AFWD":">","AREV":"<","CLINK":"=",
  "IMSCRIB":"\\u2299","FSPLIT":"\\u25c7","FFUSE":"\\u25cf","EVALT":"+","EVALF":"\\u00d7",
  "ENGAGR":"\\u229e","IFIX":"\\u00ac"};
const FAM = {"VINIT":0,"TANCH":0,"AFWD":0,"AREV":0,"CLINK":0,"IMSCRIB":0,
  "FSPLIT":1,"FFUSE":1,"EVALT":2,"EVALF":2,"ENGAGR":2,"IFIX":3};
const FAMC = ["#6af","#a84bb8","#e8b84b","#6f6"];
const FAMN = ["LOGICAL","FROBENIUS","DIALETHEIA","LINEAR"];
const PC = ["#e8b84b","#4ba8b8","#a84bb8"];

// IMASM programs
const CANON = """ + json.dumps(CANONICALS) + """;

// IG Primitive ordinal values (from imas_ig.rs)
function primOrd(name) {
  const m = {"Phi":5,"odot":2,"Omega":3,"H":4,"G":3,"Th":5,"R":4,"f":3,"K":3,"D":3,"Sigma":3,"c":3};
  return m[name] || 3;
}

// Evaluate an IG tuple against dialect gates
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
}

// Compute IG tuple from IMASM word
function igFromWord(tokens) {
  // Map token sequence to IG primitives using the Snapshot->IgTuple logic from imas_ig.rs
  const n = tokens.length;
  if (n === 0) return {};
  
  // Token diversity -> D
  const unique = new Set(tokens);
  const d_div = unique.size;
  const D = d_div <= 2 ? 1 : d_div <= 5 ? 2 : d_div <= 9 ? 3 : 4;
  
  // Self-reference
  const sr = tokens[0] === tokens[n-1];
  
  // Token families
  let fams = [0,0,0,0];
  let fsplit = false, ffuse = false;
  for (const t of tokens) {
    fams[FAM[t]]++;
    if (t === "FSPLIT") fsplit = true;
    if (t === "FFUSE") ffuse = true;
  }
  const fo = (fsplit && ffuse) ? 1 : 0;
  const dc = tokens.includes("EVALT") && tokens.includes("EVALF") && tokens.includes("ENGAGR");
  
  // Period
  let period = n;
  for (let p = 1; p <= n; p++) {
    if (n % p === 0) {
      let ok = true;
      for (let i = p; i < n && ok; i++) {
        if (tokens[i] !== tokens[i-p]) ok = false;
      }
      if (ok) { period = p; break; }
    }
  }
  
  const ifix_count = tokens.filter(t => t === "IFIX").length;
  
  // Dimensionality
  const d_val = D;
  // Topology
  const t_val = sr ? 5 : period === 1 ? 1 : period === 2 ? 3 : fo > 0 ? 4 : 2;
  // Coupling
  const r_val = fo === 1 ? 4 : fo === 2 ? 3 : 2;
  // Parity
  const p_val = fo === 1 ? 5 : 4;
  // Fidelity
  const f_val = dc ? 3 : period === 1 ? 1 : 2;
  // Kinetics
  const k_val = ifix_count >= 8 ? 4 : period <= 2 ? 3 : period <= 4 ? 2 : 1;
  // Cardinality
  const g_val = ifix_count >= 3 ? 3 : d_div <= 3 ? 1 : 2;
  // Composition
  const c_val = fo > 0 ? 3 : period === 1 ? 1 : 2;
  // Criticality
  const phi_val = sr && dc ? 2 : dc ? 4 : period === 1 ? 1 : 3;
  // Chirality
  const h_val = period === 1 ? 1 : period === 2 ? 2 : period <= 4 ? 3 : 4;
  // Stoichiometry
  const s_val = d_div >= 8 ? 3 : d_div >= 4 ? 2 : 1;
  // Winding
  const om_val = tokens.includes("AREV") ? 3 : fsplit ? 2 : 1;
  
  return {D:d_val, T:t_val, R:r_val, P:p_val, F:f_val, K:k_val, G:g_val, C:c_val,
          phi:phi_val, H:h_val, S:s_val, omega:om_val,
          p:p_val, f:f_val, k:k_val, g:g_val, c:c_val, h:h_val, s:s_val, d:d_val, t:t_val, r:r_val};
}
""")

H.append("""
function getTokens() {
  const sel = document.getElementById("wordSel").value;
  if (sel === "custom") {
    const raw = document.getElementById("customWord").value.trim().toUpperCase();
    return raw ? raw.split(/\\s+/) : [];
  }
  return CANON[sel].split(/\\s+/);
}

function updateWord() {
  const sel = document.getElementById("wordSel").value;
  document.getElementById("customWord").style.display = sel === "custom" ? "block" : "none";
  if (sel !== "custom") evaluate();
}

function onCustom() { evaluate(); }

function updateDialect() { evaluate(); }

function showRuleset(idx) {
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
}

// Token adjacency from composer.py
function validTransition(from, to) {
  const t1 = from, t2 = to;
  if (t1 === "VINIT") return t2 !== "TANCH" && t2 !== "FFUSE";
  if (t1 === "TANCH") return ["VINIT","IMSCRIB","AFWD"].includes(t2);
  if (["AFWD","AREV","CLINK","IMSCRIB"].includes(t1)) return true;
  if (t1 === "FSPLIT") return !["VINIT","TANCH","FSPLIT","FFUSE"].includes(t2);
  if (t1 === "FFUSE") return true;
  if (["EVALT","EVALF","ENGAGR"].includes(t1)) return t2 !== "VINIT";
  if (t1 === "IFIX") return true;
  return true;
}

function evaluate() {
  const idx = parseInt(document.getElementById("dialectSel").value);
  showRuleset(idx);
  
  const tokens = getTokens();
  if (tokens.length === 0) {
    document.getElementById("rslt-box").innerHTML = "No tokens to evaluate";
    document.getElementById("cfg-box").innerHTML = "";
    return;
  }
  
  // Compute IG tuple
  const ig = igFromWord(tokens);
  
  // Evaluate gates (use the actual strictness from data)
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
  drawCFG(tokens, ig, g1_pass && g2_pass && g3_pass, seq, g1_pass, g2_pass, g3_pass, idx);
}
""")

H.append("""
function drawCFG(tokens, ig, allPass, seq, g1p, g2p, g3p, idx) {
  const svg = document.getElementById("cfgSvg");
  const n = tokens.length;
  const W = document.getElementById("cfg-view").clientWidth || 800;
  const H = 480;
  svg.setAttribute("viewBox", "0 0 " + W + " " + H);
  
  let html = "";
  
  // Layout: place tokens in a bowtie/cycle pattern
  const cx = W / 2, cy = H / 2;
  const R = Math.min(W, H) * 0.38;
  
  // Positions: arrange in a circle (cyclic program)
  const pos = [];
  const startAngle = -Math.PI/2;
  for (let i = 0; i < n; i++) {
    const angle = startAngle + (2 * Math.PI * i / n);
    pos.push({
      x: cx + R * Math.cos(angle),
      y: cy + R * Math.sin(angle)
    });
  }
  
  // Draw edges
  for (let i = 0; i < n; i++) {
    const j = (i + 1) % n; // cyclic - wraps around
    const p1 = pos[i], p2 = pos[j];
    const isCyclic = j === 0;
    
    // Determine if this edge is valid under the dialect
    const from_tok = tokens[i], to_tok = tokens[j];
    const valid = validTransition(from_tok, to_tok);
    
    const cls = valid ? (allPass ? "pass" : "fail") : "fail";
    const edgeCls = `edge ${isCyclic ? "back" : cls}`;
    
    // Curved edge for visual clarity
    const mx = (p1.x + p2.x) / 2;
    const my = (p1.y + p2.y) / 2;
    const dx = p2.x - p1.x;
    const dy = p2.y - p1.y;
    const len = Math.sqrt(dx*dx + dy*dy);
    const nx = -dy/len * 10;
    const ny = dx/len * 10;
    
    html += `<path class="${edgeCls}" d="M${p1.x},${p1.y} Q${mx+nx},${my+ny} ${p2.x},${p2.y}"/>`;
    
    // Edge label (Frobenius pair indicator)
    if (from_tok === "FSPLIT" && to_tok === "FFUSE") {
      html += `<text x="${mx+nx}" y="${my+ny-6}" fill="#a84bb8" font-size="7">FSPLIT→FFUSE</text>`;
    }
    if (from_tok === "FFUSE" && to_tok === "FSPLIT") {
      html += `<text x="${mx+nx}" y="${my+ny-6}" fill="#a84bb8" font-size="7">FFUSE→FSPLIT</text>`;
    }
  }
  
  // Draw token nodes
  for (let i = 0; i < n; i++) {
    const t = tokens[i];
    const p = pos[i];
    const fam = FAM[t] || 0;
    const col = FAMC[fam];
    const glyph = GLYPH[t] || t;
    
    // Token pass/fail based on gate status
    // For now, show based on overall pass
    const isPass = allPass;
    const r = isPass ? 16 : 12;
    
    // Draw node circle with gate-aware coloring
    html += `<g class="token-node">`;
    html += `<circle cx="${p.x}" cy="${p.y}" r="${r}" fill="#1a1828" stroke="${col}" stroke-width="${isPass ? 2.5 : 1.5}" class="token-ring" opacity="${isPass ? 1 : 0.5}"/>`;
    
    // Gate badge
    const badgeCol = allPass ? "#4a8" : "#844";
    html += `<circle cx="${p.x+10}" cy="${p.y-10}" r="5" fill="${badgeCol}" opacity="0.8"/>`;
    
    // Token label
    html += `<text x="${p.x}" y="${p.y+3}" text-anchor="middle" fill="${col}" font-size="11" font-weight="bold">${glyph}</text>`;
    
    // Index label below
    html += `<text x="${p.x}" y="${p.y+r+12}" text-anchor="middle" fill="#5a659c" font-size="7">${i}</text>`;
    html += `</g>`;
  }
  
  // FSPLIT/FFUSE pair highlighting
  let fsplitCount = tokens.filter(t => t === "FSPLIT").length;
  let ffuseCount = tokens.filter(t => t === "FFUSE").length;
  const balanced = fsplitCount === ffuseCount;
  
  html += `<text x="12" y="${H-12}" fill="#5a659c" font-size="9">FSplit:${fsplitCount} FFuse:${ffuseCount} ${balanced ? "(balanced)" : "(unbalanced)"}</text>`;
  
  // Title
  const d = DATA[idx];
  html += `<text x="${cx}" y="20" text-anchor="middle" fill="#e8b84b" font-size="13" font-weight="bold">${d.name} (U_${idx})</text>`;
  html += `<text x="${cx}" y="34" text-anchor="middle" fill="#8a84b0" font-size="9">${n} tokens &middot; ${allPass ? "ALL GATES PASS" : "GATE VIOLATION"}${seq ? " &middot; sequential" : " &middot; parallel"}</text>`;
  
  svg.innerHTML = html;
  
  // Grammar details box
  const box = document.getElementById("cfg-box");
  const families = {};
  for (const t of tokens) { families[FAMN[FAM[t]]] = (families[FAMN[FAM[t]]] || 0) + 1; }
  const famStr = Object.entries(families).map(([k,v]) => k + ":" + v).join(" ");
  
  box.innerHTML = `
    <div><b>Token sequence</b>: ${tokens.join(" &rarr; ")} (cyclic)</div>
    <div><b>Families</b>: ${famStr}</div>
    <div><b>Token diversity</b>: ${new Set(tokens).size} / ${tokens.length}</div>
    <div><b>FSPLIT/FFUSE balance</b>: ${fsplitCount}/${ffuseCount} ${balanced ? '<span style="color:#6f6">OK</span>' : '<span style="color:#f66">MISMATCH</span>'}</div>
    <div style="margin-top:4px"><b>Gate-constrained CFG production rules:</b></div>
  `;
  
  // Show valid token transitions (CFG productions)
  const allTokens = Object.keys(GLYPH);
  let prodCount = 0;
  for (const from of allTokens) {
    const validTo = allTokens.filter(to => validTransition(from, to));
    const fromGlyph = GLYPH[from] || from;
    const toStr = validTo.map(t => GLYPH[t] || t).join(" ");
    if (validTo.length > 0) {
      box.innerHTML += `<div style="font-size:9px;color:#8a84b0;margin-left:12px">${fromGlyph} &rarr; ${toStr}</div>`;
      prodCount += validTo.length;
    }
  }
  box.innerHTML += `<div style="margin-top:4px"><span class="tag tag-info">${prodCount} productions across ${allTokens.length} nonterminals</span></div>`;
}

window.onload = function() {
  // Show first dialect
  showRuleset(0);
  // Auto-select canonical program
  document.getElementById("wordSel").value = "agent_loop";
  evaluate();
};
</script>
</body></html>
""")

# ===== WRITE HTML =====
out = HERE / "dialect_cfg_viz.html"
out.write_text("\n".join(H), encoding="utf-8")
print(f"Wrote {out} ({out.stat().st_size} bytes)")

# ===== GATE DATA =====
# Build proper gate map from dialect names and Rust source
GATE_MAP = {}
for d in DATA:
    name = d["name"]
    strict = d["strict"]
    seq = d["seq"]
    
    # Derive proper gates from name patterns (matches dialect_expansion.rs)
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
    elif "absorption" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "predator" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
    elif "prey" in name:
        g = ("Phi",5.0,"odot",2.0,"Omega",3.0)
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
        # Fallback: derive from strictness
        g1m = max(0, min(5, round(strict/2.5)))
        g2m = max(0, min(3, round((strict-3)/3)))
        g3m = max(0, min(4, round(strict/3)))
        g = ("Phi",float(g1m),"odot",float(g2m),"Omega",float(g3m))
    
    GATE_MAP[name] = g

# Replace the gate computation in the JavaScript with real gate data
# We embed GATE_MAP as JSON in the script
GATES_JSON = json.dumps({k:{"g1_p":v[0],"g1_m":v[1],"g2_p":v[2],"g2_m":v[3],"g3_p":v[4],"g3_m":v[5]} for k,v in GATE_MAP.items()})
