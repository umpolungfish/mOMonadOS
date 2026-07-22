#!/usr/bin/env python3
"""Generate the 88-dialect torus visualization (U_0 through U_87).
Outputs: dialect_torus.html (interactive Three.js), dialect_positions.json
"""
import json
from pathlib import Path
HERE = Path(__file__).parent

# Each entry: (name, phase, strictness, t_count, seq, desc)
# phase: 0=canonical, 1=hand-crafted, 2=expansion
D = [
# Phase 0 — Canonical (U_0..U_7)
("canonical",0,10.0,5,1,"Our dialect: Frobenius then self-modeling then winding"),
("low_gate",0,7.0,5,1,"Lowered thresholds: directional parity suffices"),
("strict_frobenius",0,11.0,5,1,"Frobenius gated by quantum fidelity"),
("inverted_gates",0,10.0,5,1,"Self-modeling precedes Frobenius"),
("no_ordering",0,10.0,5,0,"All three gates fully independent"),
("high_gate",0,11.33,5,1,"Strictest: max winding, full self-model, parity-perfect"),
("winding_first",0,10.0,5,1,"Topological order: winding first"),
("t_structural",0,10.0,5,1,"Time as geometry: T from structural primitives"),
# Phase I — Hand-crafted (U_8..U_28)
("chirality_first",1,8.0,5,1,"Memory before closure: G1=H>=2-step"),
("topology_universe",1,11.0,5,1,"Connectivity is the fundamental gate: G1=Th>=odot"),
("scope_universe",1,8.0,5,1,"Universality first: G1=G>=aleph"),
("dimensional_gate",1,10.0,5,1,"State-space is first gate: G1=D>=infty"),
("kinetics_trap",1,8.0,5,1,"Slowness is a structural requirement"),
("triple_criticality",1,6.0,5,1,"Three rungs of odot criticality"),
("t_hybrid",1,10.0,8,1,"T from 8 primitives: dynamics + geometry"),
("broadcast_universe",1,8.0,5,1,"G1=G>=seq (sequential composition)"),
("t_inverted",1,10.0,5,1,"T from structural (non-dynamic) primitives"),
("single_gate",1,7.0,5,1,"Only G1 matters; G2/G3 trivial"),
("fidelity_universe",1,10.0,5,1,"Quantum coherence is fundamental gate"),
("stoichiometry_universe",1,8.0,5,1,"Heterogeneity is the first gate"),
("absorption_democracy",1,10.0,5,1,"No absorptions -- pure lattice ops"),
("absorption_monarchy",1,10.0,5,1,"odot, Sigma, Phi, Omega all absorb"),
("absorption_inverted",1,10.0,5,1,"Trivial values absorb; complexity fragile"),
("absorption_tensor_only",1,10.0,5,1,"Absorption only under tensor"),
("absorption_chirality_first",1,8.0,5,1,"H absorbs; memory is dominant"),
("absorption_scope_empire",1,8.0,5,1,"G absorbs; universal swallows particular"),
("absorption_topology_seal",1,10.0,5,1,"Th absorbs; topology is destiny"),
("predator_universe",1,10.0,5,1,"Phi absorbs left only -- agency"),
("prey_universe",1,10.0,5,1,"Phi absorbs right only -- passivity"),
# Phase III — Expansion (U_29..U_87)
("coupling_first",2,8.0,5,1,"Relation before closure: G1=R>=adjoint"),
("coupling_maximal",2,9.0,5,1,"Only bilateral coupling suffices"),
("chirality_second",2,11.0,5,1,"After Frobenius you must remember"),
("dimensional_second",2,10.0,5,1,"After Frobenius: infinite canvas"),
("topology_second",2,10.0,5,1,"After Frobenius: topology determines trace"),
("fidelity_second",2,10.0,5,1,"After Frobenius: only quantum traces"),
("scope_second",2,10.0,5,1,"After Frobenius: universal scope"),
("composition_second",2,10.0,5,1,"After Frobenius: sequential composition"),
("winding_second",2,9.0,5,1,"After Frobenius: topological sealing"),
("kinetics_second",2,10.0,5,1,"After Frobenius: slow kinetics"),
("chirality_third",2,10.0,5,1,"Eternal memory as terminal seal"),
("dimensional_third",2,10.0,5,1,"Holographic state-space as terminal seal"),
("topology_third",2,10.0,5,1,"Box-product topology as terminal seal"),
("fidelity_third",2,10.0,5,1,"Quantum coherence as terminal seal"),
("scope_third",2,10.0,5,1,"Universal scope as terminal seal"),
("composition_third",2,10.0,5,1,"Broadcast composition as terminal seal"),
("coupling_third",2,10.0,5,1,"Bilateral coupling as terminal seal"),
("kinetics_third",2,10.0,5,1,"Slow or trapped kinetics as terminal seal"),
("parallel_canonical",2,10.0,5,0,"Canonical gates in parallel: all independent"),
("parallel_low",2,7.0,5,0,"Low gates parallel: easiest O_inf access"),
("parallel_high",2,11.33,5,0,"High gates parallel: strictest but independent"),
("parallel_chirality",2,8.0,5,0,"Chirality gates parallel: H, odot, Omega"),
("parallel_topology",2,11.0,5,0,"Topology gates parallel: Th, R, odot"),
("parallel_scope",2,8.0,5,0,"Scope gates parallel: G, odot, Omega"),
("parallel_broadcast",2,8.0,5,0,"Broadcast gates parallel: G, odot, Omega"),
("parallel_dimensional",2,10.0,5,0,"Dimensional gates parallel: D, odot, Phi"),
("parallel_kinetics",2,8.0,5,0,"Kinetics gates parallel: K, odot, Omega"),
("triple_parity",2,12.0,5,1,"Parity ladder: directional, full, Frobenius-special"),
("triple_topology",2,12.0,5,1,"Topology ladder: bowtie, box, imscriptive"),
("triple_coupling",2,11.0,5,1,"Coupling ladder: adjoint to bilateral"),
("triple_chirality",2,9.0,5,1,"Chirality ladder: 1-step, 2-step, eternal"),
("triple_winding",2,9.0,5,1,"Winding ladder: Z2, integer, non-Abelian"),
("ordinal4_parity",2,12.0,5,1,"Ordinal-4: parity rung repeated -- 4 gates"),
("ordinal4_winding",2,9.0,5,1,"Ordinal-4: winding rung repeated -- 4 gates"),
("ordinal_swap",2,9.0,5,1,"Swapped order: winding first, odot, parity"),
("ordinal_invert",2,10.0,5,1,"Inverted: stoichiometry, parity, odot"),
("t_ceiling_topology",2,9.0,1,1,"T-ceiling: Th capped at box product"),
("t_ceiling_dimensional",2,10.0,2,1,"T-ceiling: D finite, Th at crossing point"),
("absorb_ep",2,9.0,5,1,"EP absorption: odot + EP rule enforced"),
("absorb_sub",2,6.0,5,1,"Sub-critical: odot absorbs sub-critical"),
("absorb_dual",2,10.0,5,1,"Dual: EP + sub-critical both absorb"),
("t_subset_th_sigma",2,10.0,2,0,"T: self-ref topology + heterogeneous (parallel)"),
("t_subset_th_omega",2,10.0,2,0,"T: self-ref topology + winding (parallel)"),
("t_subset_d_sigma",2,10.0,2,0,"T: infinite-dim + heterogeneous (parallel)"),
("mixed_gamma_th",2,10.0,5,1,"Universal range before self-ref topology"),
("mixed_fidelity_coupling",2,9.0,5,1,"Quantum before bilateral coupling"),
("mixed_composition_kinetics",2,8.0,5,1,"Sequential before slow kinetics"),
("g4_quad",2,10.0,5,1,"Quad-gate: G, Phi, odot, Omega"),
("only_parity",2,5.0,5,0,"Single gate: Phi>=pmsym only"),
("only_winding",2,3.0,5,0,"Single gate: Omega>=Z only"),
("only_odot",2,2.0,5,0,"Single gate: odot>=c only"),
("empty_gate",2,0.0,5,0,"Zero gates -- all 17.28M types pass"),
("dense_gates",2,12.0,5,1,"5 distinct primitives across 3 gate slots"),
("chirality_winding",2,9.0,5,1,"Eternal memory + topological protection"),
("composition_scope",2,8.0,5,1,"Sequential composition across all scales"),
("fidelity_chirality",2,8.0,5,1,"Quantum enables structured memory"),
("t_broadcast",2,9.0,1,1,"T from broadcast composition (G>=broad)"),
("absorb_broadcast",2,11.0,1,1,"EP absorption + broadcast T"),
("the_all",2,10.0,12,1,"odot->Phi->Omega->Th + 12-primitive T"),
]

assert len(D)==88, f"Expected 88, got {len(D)}"

import numpy as np
N = len(D)
R = r = 2.0
strictness = np.array([d[2] for d in D])
s_min, s_max = strictness.min(), strictness.max()
s_rng = s_max - s_min if s_max != s_min else 1.0
theta = np.linspace(0, 2*np.pi, N, endpoint=False)
phi = np.pi + np.pi/3 * (2*(strictness-s_min)/s_rng - 1)
xs = ((R+r*np.cos(phi))*np.cos(theta)).tolist()
ys = ((R+r*np.cos(phi))*np.sin(theta)).tolist()
zs = (r*np.sin(phi)).tolist()

PHASE_COL = ['#e8b84b','#4ba8b8','#a84bb8']
PHASE_NM = ['Phase 0 — Canonical','Phase I — Hand-crafted','Phase III — Expansion']

data = []
for i,(name,phase,strict,tcnt,seq,desc) in enumerate(D):
    data.append({'i':i,'name':name,'phase':phase,'strict':round(strict,2),
        'tcnt':tcnt,'seq':seq,'desc':desc,
        'x':round(xs[i],4),'y':round(ys[i],4),'z':round(zs[i],4),
        'theta':round(theta[i],4),'phi':round(phi[i],4)})

html = r"""<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1.0">
<title>Dialect Torus -- U_0 through U_87</title><style>
*{margin:0;padding:0;box-sizing:border-box}
body{background:#0b0b12;overflow:hidden;font-family:'Courier New',monospace}
#tip{position:absolute;display:none;pointer-events:none;background:rgba(11,11,18,0.94);
  border:1px solid #5a659c;border-radius:6px;padding:10px 14px;color:#f0ead8;
  font-size:12px;max-width:340px;line-height:1.5;z-index:100}
#tip .i{color:#8a84b0;font-size:10px}
#tip .n{color:#f0ead8;font-weight:bold;font-size:14px}
#tip .d{color:#b0a8c0;font-size:11px;margin-top:4px}
#tip .p{display:inline-block;width:10px;height:10px;border-radius:50%;margin-right:6px;vertical-align:middle}
#leg{position:absolute;top:20px;right:20px;color:#b0a8c0;font-size:12px;
  background:rgba(11,11,18,0.88);padding:14px 18px;border-radius:8px;border:1px solid #3a3550;line-height:2}
#leg .d{display:inline-block;width:10px;height:10px;border-radius:50%;margin-right:8px;vertical-align:middle}
#leg .s{margin-top:8px;padding-top:6px;border-top:1px solid #3a3550;font-size:10px;color:#8a84b0}
#ttl{position:absolute;top:20px;left:50%;transform:translateX(-50%);color:#f0ead8;font-size:20px;
  font-weight:bold;text-align:center;text-shadow:0 0 20px rgba(232,184,75,0.2)}
#ttl .sb{font-size:12px;color:#8a84b0;font-weight:normal;margin-top:4px}
</style></head><body>
<div id="ttl">Dialect Torus<div class="sb">88 rulesets on the horn torus &middot; U_0 through U_87</div></div>
<div id="leg">"""

for pi in range(3):
    html += f'<div><span class="d" style="background:{PHASE_COL[pi]}"></span>{PHASE_NM[pi]}</div>\n'
html += '<div class="s">size &prop; gate strictness &middot; &phi; offset &prop; strictness<br>hover for details &middot; drag to orbit</div></div>\n'
html += '<div id="tip"></div>\n'
html += '<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>\n'
html += '<script src="https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/controls/OrbitControls.js"></script>\n'
html += '<script>\nvar data = ' + json.dumps(data) + ';\n'

html += r"""
const R=2.0,r=2.0;
var scene=new THREE.Scene(); scene.background=new THREE.Color(0x0b0b12);
var cam=new THREE.PerspectiveCamera(45,window.innerWidth/window.innerHeight,0.1,100);
cam.position.set(6,4,8);
var ren=new THREE.WebGLRenderer({antialias:true});
ren.setSize(window.innerWidth,window.innerHeight);
ren.setPixelRatio(Math.min(window.devicePixelRatio,2));
document.body.appendChild(ren.domElement);
var ctl=new THREE.OrbitControls(cam,ren.domElement);
ctl.enableDamping=true; ctl.dampingFactor=0.08; ctl.minDistance=3; ctl.maxDistance=20;
ctl.autoRotate=true; ctl.autoRotateSpeed=0.8;
// horn torus surface
var tGeom=new THREE.TorusGeometry(2,2,40,80);
var sMat=new THREE.MeshPhongMaterial({color:0x28304f,transparent:true,opacity:0.28,side:THREE.DoubleSide});
var surf=new THREE.Mesh(tGeom,sMat); scene.add(surf);
var wMat=new THREE.MeshBasicMaterial({color:0x5a659c,wireframe:true,transparent:true,opacity:0.12});
var wire=new THREE.Mesh(tGeom.clone(),wMat); scene.add(wire);
// lighting
scene.add(new THREE.AmbientLight(0x404060));
var d1=new THREE.DirectionalLight(0xffeedd,1.0); d1.position.set(5,8,6); scene.add(d1);
var d2=new THREE.DirectionalLight(0x4466ff,0.5); d2.position.set(-4,-2,5); scene.add(d2);
// phase colors
var pcol=["#e8b84b","#4ba8b8","#a84bb8"];
var phN=["Phase 0 -- Canonical","Phase I -- Hand-crafted","Phase III -- Expansion"];
// strictness scale
var ss=data.map(function(d){return d.strict;});
var sMin=Math.min.apply(null,ss), sMax=Math.max.apply(null,ss), sRng=sMax-sMin||1;
// place dialect stations
var sg=new THREE.Group(); scene.add(sg);
var rc=new THREE.Raycaster(); var ms=new THREE.Vector2();
var tip=document.getElementById('tip');
var meshes=[]; var hoveredId=-1;
for(var i=0;i<data.length;i++){
  var d=data[i];
  var sz=0.15+0.30*(d.strict-sMin)/sRng;
  var col=new THREE.Color(pcol[d.phase]);
  var sp=new THREE.SphereGeometry(sz,16,12);
  var mat=new THREE.MeshPhongMaterial({color:col,emissive:col,emissiveIntensity:0.15});
  var mesh=new THREE.Mesh(sp,mat);
  mesh.position.set(d.x,d.y,d.z);
  mesh.userData={idx:d.i};
  sg.add(mesh); meshes.push(mesh);
}
// sequential edge ribbon
var ePts=[];
for(var i=0;i<data.length;i++){var d=data[i];ePts.push(new THREE.Vector3(d.x,d.y,d.z));}
var eGeom=new THREE.BufferGeometry().setFromPoints(ePts);
var eMat=new THREE.LineBasicMaterial({color:0x5a659c,transparent:true,opacity:0.35});
scene.add(new THREE.Line(eGeom,eMat));
// closing edge U_87 -> U_0
var cGeom=new THREE.BufferGeometry().setFromPoints([
  new THREE.Vector3(data[87].x,data[87].y,data[87].z),
  new THREE.Vector3(data[0].x,data[0].y,data[0].z)]);
var cLine=new THREE.Line(cGeom,new THREE.LineBasicMaterial({color:0x5a659c,transparent:true,opacity:0.25}));
scene.add(cLine);
// pinch point (origin)
var pg=new THREE.SphereGeometry(0.10,16,12);
var pMat=new THREE.MeshPhongMaterial({color:0xffffff,emissive:0xe8b84b,emissiveIntensity:0.4});
var pinch=new THREE.Mesh(pg,pMat); pinch.position.set(0,0,0); scene.add(pinch);
// U symbol label at pinch
var ucan=document.createElement('canvas'); ucan.width=120; ucan.height=50;
var uctx=ucan.getContext('2d');
uctx.clearRect(0,0,120,50);
uctx.fillStyle='#e8b84b'; uctx.font='bold 30px "Courier New"'; uctx.textAlign='center';
uctx.fillText('U',60,32);
uctx.fillStyle='rgba(232,184,75,0.35)'; uctx.font='12px "Courier New"';
uctx.fillText('88 dialects',60,48);
var utexture=new THREE.CanvasTexture(ucan); utexture.minFilter=THREE.LinearFilter;
var usprite=new THREE.Sprite(new THREE.SpriteMaterial({map:utexture,transparent:true,depthTest:false}));
usprite.position.set(0,0,-1.6); usprite.scale.set(1.2,0.5,1); scene.add(usprite);
// hover handler
ren.domElement.addEventListener('pointermove',function(e){
  var rect=ren.domElement.getBoundingClientRect();
  ms.x=((e.clientX-rect.left)/rect.width)*2-1;
  ms.y=-((e.clientY-rect.top)/rect.height)*2+1;
  rc.setFromCamera(ms,cam);
  var hits=rc.intersectObjects(meshes);
  if(hits.length>0){
    var obj=hits[0].object;
    var idx=obj.userData.idx;
    if(idx!==hoveredId){hoveredId=idx;ren.domElement.style.cursor='pointer';}
    var d=data[idx];
    tip.style.display='block';
    tip.style.left=(e.clientX+14)+'px'; tip.style.top=(e.clientY-10)+'px';
    tip.innerHTML='<span class="p" style="background:'+pcol[d.phase]+'"></span><span class="n">U_'+d.i+' -- '+d.name+'</span>'+
      '<br><span class="i">'+phN[d.phase]+' &middot; strictness '+d.strict+' &middot; '+(d.seq?'seq':'par')+'</span>'+
      '<br><span class="d">'+d.desc+'</span>';
  } else {
    hoveredId=-1; tip.style.display='none'; ren.domElement.style.cursor='default';
  }
});
window.addEventListener('resize',function(){cam.aspect=window.innerWidth/window.innerHeight;cam.updateProjectionMatrix();ren.setSize(window.innerWidth,window.innerHeight);});
function anim(){requestAnimationFrame(anim);ctl.update();ren.render(scene,cam);}
anim();
</script></body></html>"""

out = HERE/'dialect_torus.html'
out.write_text(html)
print(f'wrote {out} ({len(html)} bytes)')

jout = HERE/'dialect_positions.json'
jout.write_text(json.dumps(data, indent=2))
print(f'wrote {jout} ({jout.stat().st_size} bytes)')
print(f'Strictness: [{s_min:.1f}, {s_max:.1f}]')
