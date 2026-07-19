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

R, r = 3.0, 1.0

fig = plt.figure(figsize=(12, 9), facecolor="#0b0b12")
ax = fig.add_subplot(111, projection="3d")
ax.set_facecolor("#0b0b12")

# ── the torus surface ───────────────────────────────────────────────────
theta = np.linspace(0, 2 * np.pi, 120)
phi = np.linspace(0, 2 * np.pi, 60)
TH, PH = np.meshgrid(theta, phi)
X = (R + r * np.cos(PH)) * np.cos(TH)
Y = (R + r * np.cos(PH)) * np.sin(TH)
Z = r * np.sin(PH)
ax.plot_surface(X, Y, Z, color="#28304f", alpha=0.55, linewidth=0, antialiased=True,
                rstride=1, cstride=1, shade=True)
ax.plot_wireframe(X, Y, Z, color="#5a659c", alpha=0.4, linewidth=0.4,
                  rstride=5, cstride=6)

# ── the kernel loop: (1,1) winding, one station per token ───────────────
t_curve = np.linspace(0, 2 * np.pi, 600)
xc = (R + r * np.cos(t_curve)) * np.cos(t_curve)
yc = (R + r * np.cos(t_curve)) * np.sin(t_curve)
zc = r * np.sin(t_curve)
ax.plot(xc, yc, zc, color="#e8b84b", linewidth=2.6, alpha=0.95)

# token stations
ang = np.array([2 * np.pi * i / n for i in range(n)])
xs = (R + r * np.cos(ang)) * np.cos(ang)
ys = (R + r * np.cos(ang)) * np.sin(ang)
zs = r * np.sin(ang)
ax.scatter(xs, ys, zs, color="#f5e6bf", s=52, depthshade=False, zorder=5)

for i, (name, a) in enumerate(zip(tokens, ang)):
    # label just outside the tube, along the outward normal
    lx = (R + 1.45 * r * np.cos(a)) * np.cos(a)
    ly = (R + 1.45 * r * np.cos(a)) * np.sin(a)
    lz = 1.45 * r * np.sin(a)
    ax.text(lx, ly, lz, f"{GLYPH[name]}\n{name}", color="#f0ead8",
            fontsize=10, ha="center", va="center", zorder=6)

# the seal-to-opening junction: TANCH meets VINIT where the loop closes
ax.scatter([xs[0]], [ys[0]], [zs[0]], color="#e8524b", s=130,
           depthshade=False, zorder=7, marker="o", facecolors="none",
           linewidths=2.2)
ax.text(xs[0] * 1.28, ys[0] * 1.28, zs[0] + 0.55, "⊣⊢  seal touches opening\n(B is the only bifurcation point)",
        color="#e8968f", fontsize=9, ha="center", zorder=7)

ax.set_title("mOMonadOS kernel — agent_loop wound on the torus\n" + " ".join(word),
             color="#f0ead8", fontsize=15, pad=18)
ax.set_box_aspect((1, 1, 0.5))
ax.set_axis_off()
ax.view_init(elev=52, azim=-58)

out = Path.home() / "mOMonadOS" / "kernel_toroidal_loop.png"
fig.savefig(out, dpi=200, bbox_inches="tight", facecolor=fig.get_facecolor())
print(f"wrote {out}  ({n} tokens: {word})")
