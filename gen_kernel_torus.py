"""The mOMonadOS kernel wound on the torus.

Computes from canonical: the token sequence is read from modot.agent.agent_loop()
(the kernel program itself), never hand-lettered. The loop is drawn as a (1,1)
winding — one toroidal revolution carrying one poloidal revolution — with the
kernel's tokens at their stations and the TANCH→VINIT seal-to-opening junction
marked (legal by the bifurcation law: the fuse's landing on B IS the run on B).
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path.home() / "imsgct" / "MoDoT"))
import numpy as np
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

from modot.agent import agent_loop

GLYPH = {
    "VINIT": "⊢", "TANCH": "⊣", "AFWD": ">", "AREV": "<", "CLINK": "=",
    "IMSCRIB": "⊙", "FSPLIT": "◇", "FFUSE": "●", "EVALT": "+", "EVALF": "×",
    "ENGAGR": "⊞", "IFIX": "¬",
}

tokens = [t.name for t in agent_loop().tokens]
n = len(tokens)
word = "".join(GLYPH[t] for t in tokens)

# Horn torus: R = r, so the inner equator collapses to the origin — the PINCH.
# A (1,1) winding passes through that pinch once per revolution, and IMSCRIB ⊙
# (the identity, the Frobenius closure μ∘δ=id) is seated THERE, at the center,
# not as one bead among twelve. Everything else winds around and through it.
R = r = 2.0

# Rotate the parametrization so IMSCRIB's station lands exactly at the pinch.
# The pinch is at poloidal/toroidal angle t = π (there (R + r cos π) = 0). The
# stations are at t_i = 2π i/n + off; solve off so t at IMSCRIB's index = π.
imscrib_i = tokens.index("IMSCRIB")
off = np.pi - 2 * np.pi * imscrib_i / n
ang = np.array([2 * np.pi * i / n + off for i in range(n)])

def horn(t):
    return ((R + r * np.cos(t)) * np.cos(t),
            (R + r * np.cos(t)) * np.sin(t),
            r * np.sin(t))

fig = plt.figure(figsize=(12, 9), facecolor="#0b0b12")
ax = fig.add_subplot(111, projection="3d")
ax.set_facecolor("#0b0b12")

# ── the horn-torus surface ──────────────────────────────────────────────
theta = np.linspace(0, 2 * np.pi, 160)
phi = np.linspace(0, 2 * np.pi, 80)
TH, PH = np.meshgrid(theta, phi)
X = (R + r * np.cos(PH)) * np.cos(TH)
Y = (R + r * np.cos(PH)) * np.sin(TH)
Z = r * np.sin(PH)
ax.plot_surface(X, Y, Z, color="#28304f", alpha=0.45, linewidth=0, antialiased=True,
                rstride=1, cstride=1, shade=True)
ax.plot_wireframe(X, Y, Z, color="#5a659c", alpha=0.35, linewidth=0.4,
                  rstride=6, cstride=7)

# ── the kernel loop: (1,1) winding, threading the pinch ─────────────────
t_curve = np.linspace(0, 2 * np.pi, 800) + off
xc, yc, zc = horn(t_curve)
ax.plot(xc, yc, zc, color="#e8b84b", linewidth=2.6, alpha=0.95)

# token stations (IMSCRIB's lands on the origin by construction)
xs, ys, zs = horn(ang)
ax.scatter(xs, ys, zs, color="#f5e6bf", s=52, depthshade=False, zorder=5)

# Labels on a clean outer ring, each at its own toroidal angle (= t for a (1,1)
# winding), so the crowding of stations near the pinch never crowds the text.
# A thin leader line ties each label back to its station.
label_R = R + 1.9 * r
for name, a, x, y, z in zip(tokens, ang, xs, ys, zs):
    if name == "IMSCRIB":
        continue  # drawn as the central node below
    lx, ly = label_R * np.cos(a), label_R * np.sin(a)
    lz = 1.9 * r * np.sin(a)
    ax.plot([x, lx], [y, ly], [z, lz], color="#5a659c", linewidth=0.6, alpha=0.5)
    ax.text(lx, ly, lz, f"{GLYPH[name]}\n{name}", color="#f0ead8",
            fontsize=10, ha="center", va="center", zorder=6)

# ── IMSCRIB ⊙ : the central node at the pinch (origin) ──────────────────
ax.scatter([0], [0], [0], color="#ffffff", s=420, depthshade=False, zorder=10,
           edgecolors="#e8b84b", linewidths=2.6)
ax.text(0, 0, -1.35, "⊙  IMSCRIB", color="#ffffff", fontsize=14, fontweight="bold",
        ha="center", va="center", zorder=11)
ax.text(0, 0, -1.95, "the pinch · μ∘δ=id · identity\nevery winding threads it",
        color="#e8b84b", fontsize=8.5, ha="center", va="center", zorder=11)

# the seal-to-opening junction: TANCH meets VINIT where the loop closes
vinit_i = tokens.index("VINIT")
ax.scatter([xs[vinit_i]], [ys[vinit_i]], [zs[vinit_i]], color="#e8524b", s=130,
           depthshade=False, zorder=7, marker="o", facecolors="none",
           linewidths=2.2)
ax.text(-(label_R + 1.6), 1.4, 2.2,
        "⊣⊢  seal touches opening\n(B is the only bifurcation point)",
        color="#e8968f", fontsize=8.5, ha="center", zorder=7)

ax.set_title("mOMonadOS kernel — agent_loop wound on the horn torus\nIMSCRIB ⊙ at the pinch  ·  " + " ".join(word),
             color="#f0ead8", fontsize=14, pad=18)
ax.set_box_aspect((1, 1, 0.62))
ax.set_axis_off()
ax.view_init(elev=26, azim=-64)

out = Path.home() / "mOMonadOS" / "kernel_toroidal_loop.png"
fig.savefig(out, dpi=200, bbox_inches="tight", facecolor=fig.get_facecolor())
print(f"wrote {out}  ({n} tokens: {word})")
