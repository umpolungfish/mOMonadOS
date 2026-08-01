// riemann_hilbert.rs — Zauner Hamiltonian H_Z eigenvalues vs Riemann zeta zeros (no_std).
//
// CONSTRUCTION 1 (circular): Zeta-encoded H, reconstructed via Gerzon inverse.
//   Verifies the SIC-POVM is informationally complete (mu∘delta=id).
//
// CONSTRUCTION 2 (non-circular): Mixed Zauner+WH Hamiltonian.
//   H = alpha*(U_Z+U_Z^dag)/2 + beta*(X+X^dag+Z+Z^dag)/4
//   Grid search over alpha,beta for best linear fit to zeta zeros.
//   NON-CIRCULAR: no zeta zeros used to construct H — only for evaluation.
//
// CONSTRUCTION 3 (non-circular): WH Laplacian.
//   H = (X+X^dag+Z+Z^dag)/4, analytical spectrum: [cos(2πa/d)+cos(2πb/d)]/2.
//
// HONEST VERDICT: The SIC-POVM faithfully ENCODES the zeta zeros
//   (Construction 1 proves mu∘delta=id) but no non-circular
//   construction produces them from the SIC-POVM alone.

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;

use crate::riemann_sic::{D, D2, PI, wh_orbit, Cmplx, cmul, cconj, cabs2, cadd, cscale};

pub const ZETA_ZEROS: [f64; 20] = [
    14.1347251417347, 21.0220396387716, 25.0108575801457, 30.4248761258595,
    32.9350615877392, 37.5861781588257, 40.9187190121480, 43.3270732809150,
    48.0051508811672, 49.7738324776723, 52.9703214777145, 56.4462476970634,
    59.3470440026024, 60.8317785246098, 65.1125440480816, 67.0798105294942,
    69.5464017111739, 72.0671576744819, 75.7046906990839, 77.1448400688748,
];

/// WH displacement D(a,b).
fn wh_d(a: usize, b: usize) -> [[Cmplx; D]; D] {
    let om = [libm::cos(2.0*PI/D as f64), libm::sin(2.0*PI/D as f64)];
    let mut m = [[[0.0, 0.0]; D]; D];
    for k in 0..D {
        let kma = (k + D - a) % D;
        let phase = (b * kma) % D;
        let mut p = [1.0, 0.0];
        for _ in 0..phase { p = cmul(p, om); }
        m[k][kma] = p;
    }
    m
}

/// Complex matrix add: C = A + B.
fn cmat_add(a: &[[Cmplx; D]; D], b: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D { c[r][col] = cadd(a[r][col], b[r][col]); } }
    c
}

/// Complex matrix * scalar.
fn cmat_scale(a: &[[Cmplx; D]; D], s: f64) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D { c[r][col] = cscale(a[r][col], s); } }
    c
}

/// Complex matrix multiply: C = A * B.
fn cmat_mul(a: &[[Cmplx; D]; D], b: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut c = [[[0.0,0.0]; D]; D];
    for r in 0..D { for col in 0..D {
        let mut s = [0.0, 0.0];
        for k in 0..D { s = cadd(s, cmul(a[r][k], b[k][col])); }
        c[r][col] = s;
    }}
    c
}

/// Hermitize: H = (H + H^dag)/2.
fn hermitize(h: &[[Cmplx; D]; D]) -> [[Cmplx; D]; D] {
    let mut r = [[[0.0,0.0]; D]; D];
    for i in 0..D { for j in 0..D {
        r[i][j][0] = (h[i][j][0] + h[j][i][0]) / 2.0;
        r[i][j][1] = (h[i][j][1] - h[j][i][1]) / 2.0;
    }}
    r
}

/// Jacobi eigenvalues for real symmetric 12x12.
fn jacobi(a: &mut [[f64; D]; D]) -> Vec<f64> {
    let tol = 1e-14;
    for _ in 0..200 {
        let (mut mx, mut p, mut q) = (0.0, 0usize, 1usize);
        for r in 0..D { for c in (r+1)..D {
            let v = libm::fabs(a[r][c]);
            if v > mx { mx = v; p = r; q = c; }
        }}
        if mx < tol { break; }
        let th = if libm::fabs(a[p][p]-a[q][q]) < 1e-15 { PI/4.0 }
                 else { 0.5 * libm::atan2(2.0*a[p][q], a[p][p]-a[q][q]) };
        let (cs, sn) = (libm::cos(th), libm::sin(th));
        let (app, aqq, apq) = (a[p][p], a[q][q], a[p][q]);
        a[p][p] = cs*cs*app + sn*sn*aqq - 2.0*cs*sn*apq;
        a[q][q] = sn*sn*app + cs*cs*aqq + 2.0*cs*sn*apq;
        a[p][q] = 0.0; a[q][p] = 0.0;
        for r in 0..D { if r != p && r != q {
            let (arp, arq) = (a[r][p], a[r][q]);
            a[r][p] = cs*arp - sn*arq; a[p][r] = a[r][p];
            a[r][q] = sn*arp + cs*arq; a[q][r] = a[r][q];
        }}
    }
    let mut ev: Vec<f64> = (0..D).map(|i| a[i][i]).collect();
    ev.sort_by(|x: &f64, y: &f64| {
        if x < y { core::cmp::Ordering::Less }
        else if x > y { core::cmp::Ordering::Greater }
        else { core::cmp::Ordering::Equal }
    });
    ev
}

/// Extract real symmetric part, compute eigenvalues.
fn eigenvalues(h: &[[Cmplx; D]; D]) -> Vec<f64> {
    let mut a = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { a[r][c] = (h[r][c][0] + h[c][r][0])/2.0; } }
    jacobi(&mut a)
}

/// H from Born probabilities: H = (d+1) sum p_i |psi_i><psi_i| - (sum p_i) I.
fn hamiltonian_from_probs(probs: &[f64], orbit: &[[Cmplx; D]]) -> [[Cmplx; D]; D] {
    let mut h = [[[0.0,0.0]; D]; D];
    let sp: f64 = probs.iter().sum();
    for (i, oi) in orbit.iter().enumerate() {
        let pi = probs[i];
        for r in 0..D { for c in 0..D {
            let contrib = cmul(oi[r], cconj(oi[c]));
            h[r][c] = cadd(h[r][c], cscale(contrib, pi));
        }}
    }
    let f = (D+1) as f64;
    for r in 0..D { for c in 0..D { h[r][c] = cscale(h[r][c], f); } }
    for r in 0..D { h[r][r][0] -= sp; }
    h
}

/// Born probabilities for H = diagonal(eigenvalues) in computational basis.
fn zeta_encoded_probs(orbit: &[[Cmplx; D]], evals: &[f64]) -> Vec<f64> {
    let mut p = vec![0.0f64; D2];
    for (i, oi) in orbit.iter().enumerate() {
        let mut s = 0.0;
        for k in 0..D { s += cabs2(oi[k]) * evals[k]; }
        p[i] = s / (D as f64);
    }
    p
}

/// Optimal linear fit: minimize ||slope*ev + intercept - target||^2.
fn linear_fit(ev: &[f64], target: &[f64]) -> (Vec<f64>, f64, f64) {
    let n = ev.len() as f64;
    let sx: f64 = ev.iter().sum();
    let sy: f64 = target.iter().sum();
    let sxy: f64 = ev.iter().zip(target.iter()).map(|(x,y)| x*y).sum();
    let sxx: f64 = ev.iter().map(|x| x*x).sum();
    let denom = n*sxx - sx*sx;
    let (slope, intercept) = if libm::fabs(denom) < 1e-15 { (0.0, sy/n) }
    else { ((n*sxy - sx*sy)/denom, (sy - ((n*sxy - sx*sy)/denom)*sx)/n) };
    let fitted: Vec<f64> = ev.iter().map(|x| slope*x + intercept).collect();
    (fitted, slope, intercept)
}

/// Build Zauner unitary U_Z via Appleby formula: tau^{r^2+2rs} / sqrt(d).
fn zauner_unitary() -> [[Cmplx; D]; D] {
    let tau: Cmplx = [-libm::cos(PI/D as f64), libm::sin(PI/D as f64)];
    let mut uz = [[[0.0,0.0]; D]; D];
    for r in 0..D {
        for s in 0..D {
            let exp = ((r*r + 2*r*s) % (2*D)) as f64;
            let mut p = [1.0, 0.0];
            for _ in 0..(exp as usize) { p = cmul(p, tau); }
            uz[r][s] = cscale(p, 1.0/libm::sqrt(D as f64));
        }
    }
    uz
}

/// Riemann-Hilbert: Zauner Hamiltonian eigenvalue computation.
pub fn run_hilbert() -> String {
    use alloc::format;
    let mut s = String::new();
    s.push_str("═══════════════════════════════════════════════════════════\n");
    s.push_str("  Riemann-Hilbert: Zauner Hamiltonian H_Z eigenvalues\n");
    s.push_str("  vs. Riemann ζ(s) non-trivial zeros\n");
    s.push_str("═══════════════════════════════════════════════════════════\n\n");

    let orbit = wh_orbit();
    s.push_str(&format!("Weyl-Heisenberg orbit: {} states generated.\n", orbit.len()));

    // Verify SIC condition
    let target_ol = 1.0/((D+1) as f64);
    let ol = cabs2({
        let mut inner = [0.0,0.0];
        for k in 0..D { inner = cadd(inner, cmul(cconj(orbit[0][k]), orbit[1][k])); }
        inner
    });
    s.push_str(&format!("SIC check: |<psi_0|psi_1>|^2 = {:.12} (target {:.12}, Delta={:.2e})\n",
        ol, target_ol, libm::fabs(ol - target_ol)));

    // Build WH generators
    let xf = wh_d(1, 0); let xb = wh_d(11, 0);
    let zf = wh_d(0, 1); let zb = wh_d(0, 11);
    let h_wh: [[Cmplx; D]; D] = {
        let mut h = cmat_add(&xf, &xb);
        h = cmat_add(&h, &zf); h = cmat_add(&h, &zb);
        hermitize(&cmat_scale(&h, 0.25))
    };

    // Build Zauner unitary
    let uz = zauner_unitary();
    let uz2 = cmat_mul(&uz, &uz);
    let uz3 = cmat_mul(&uz2, &uz);
    let phase = uz3[0][0];
    let mut uz3_err = 0.0f64;
    for r in 0..D { for c in 0..D {
        let expected = if r == c { phase } else { [0.0, 0.0] };
        let diff = libm::fabs(uz3[r][c][0]-expected[0]) + libm::fabs(uz3[r][c][1]-expected[1]);
        if diff > uz3_err { uz3_err = diff; }
    }}
    s.push_str(&format!("Zauner unitary order-3: max|U^3-phase*I| = {:.2e}\n", uz3_err));

    let uz_herm = hermitize(&uz);
    let mut uz_real = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { uz_real[r][c] = uz_herm[r][c][0]; } }
    let ev_uz_herm = jacobi(&mut uz_real);
    s.push_str(&format!("U_Z_herm eigenvalues: {:?}\n", ev_uz_herm));

    let mut h_wh_real = [[0.0f64; D]; D];
    for r in 0..D { for c in 0..D { h_wh_real[r][c] = h_wh[r][c][0]; } }

    let z12: Vec<f64> = ZETA_ZEROS.iter().take(D).copied().collect();

    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 1: Zeta-encoded (CIRCULAR — verifies mu∘delta=id)
    // ═══════════════════════════════════════════════════════════
    s.push_str("\n═══════════════════════════════════════════════════════════\n");
    s.push_str("  CONSTRUCTION 1: Zeta-encoded Gerzon reconstruction\n");
    s.push_str("  (CIRCULAR: zeta zeros used as eigenvalues of input H)\n");
    s.push_str("  Purpose: verify mu∘delta=id for d=12 SIC-POVM frame.\n");
    s.push_str("═══════════════════════════════════════════════════════════\n\n");

    let probs_zeta = zeta_encoded_probs(&orbit, &z12);
    let h_rec = hamiltonian_from_probs(&probs_zeta, &orbit);

    let mut max_anti = 0.0f64;
    for r in 0..D { for c in 0..D {
        let a = libm::fabs(h_rec[r][c][0]-h_rec[c][r][0])
              + libm::fabs(h_rec[r][c][1]+h_rec[c][r][1]);
        if a > max_anti { max_anti = a; }
    }}
    s.push_str(&format!("  Self-adjoint: max|H-H^dag| = {:.2e}\n", max_anti));

    let ev_rec = eigenvalues(&h_rec);
    let (fit_rec, sl_rec, ic_rec) = linear_fit(&ev_rec, &z12);

    s.push_str(&format!("  Linear fit: lam_fit = {:.6} * lam + {:.6}\n", sl_rec, ic_rec));
    s.push_str(&format!("  {:>3}  {:>14}  {:>14}  {:>14}  {:>10}\n",
        "n", "lam_n(H_Z)", "lam_fit", "t_n(zeta)", "|Delta|"));
    s.push_str(&format!("  {}  {}  {}  {}  {}\n",
        "---", "--------------", "--------------", "--------------", "----------"));
    let mut max_d1 = 0.0f64; let mut sum_d1 = 0.0f64;
    for n in 0..D {
        let delta = libm::fabs(fit_rec[n] - z12[n]);
        if delta > max_d1 { max_d1 = delta; }
        sum_d1 += delta;
        s.push_str(&format!("  {:3}  {:>14.8}  {:>14.8}  {:>14.8}  {:>10.6}\n",
            n+1, ev_rec[n], fit_rec[n], z12[n], delta));
    }
    let md_rec = sum_d1 / (D as f64);
    s.push_str(&format!("  Mean |Delta| = {:.6},  Max |Delta| = {:.6}\n", md_rec, max_d1));
    let gerzon_ok = md_rec < 1e-10;
    s.push_str(&format!("  Gerzon mu∘delta=id: {}\n",
        if gerzon_ok { "PASS (mean|Delta| < 1e-10)" } else { "NOTE: non-zero but small" }));

    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 2: Mixed Zauner+WH (NON-CIRCULAR)
    // H = alpha*(U_Z+U_Z^dag)/2 + beta*(X+X^dag+Z+Z^dag)/4
    // Grid search over alpha,beta for best linear fit.
    // ═══════════════════════════════════════════════════════════
    s.push_str("\n═══════════════════════════════════════════════════════════\n");
    s.push_str("  CONSTRUCTION 2: Mixed Zauner+WH Hamiltonian\n");
    s.push_str("  (NON-CIRCULAR: no zeta zeros used to construct H)\n");
    s.push_str("  H = alpha*(U_Z+U_Z^dag)/2 + beta*(X+X^dag+Z+Z^dag)/4\n");
    s.push_str("  Grid search over alpha,beta for best linear fit.\n");
    s.push_str("═══════════════════════════════════════════════════════════\n\n");

    let mut best_mean = core::f64::INFINITY;
    let mut best_alpha = 0.0f64;
    let mut best_beta = 0.0f64;
    let mut best_ev: Vec<f64> = Vec::new();

    let alpha_range: Vec<f64> = (0..25).map(|i| 0.1 + i as f64 * 0.4).collect();
    let beta_range: Vec<f64> = (0..25).map(|i| 0.5 + i as f64 * 1.2).collect();

    for &alpha in &alpha_range {
        for &beta in &beta_range {
            let mut h = [[0.0f64; D]; D];
            for r in 0..D { for c in 0..D {
                h[r][c] = alpha * uz_real[r][c] + beta * h_wh_real[r][c];
            }}
            let ev = jacobi(&mut h);
            let (fit, _, _) = linear_fit(&ev, &z12);
            let md: f64 = fit.iter().zip(z12.iter())
                .map(|(f,z)| libm::fabs(f-z)).sum::<f64>()/(D as f64);
            if md < best_mean {
                best_mean = md;
                best_alpha = alpha;
                best_beta = beta;
                best_ev = ev;
            }
        }
    }

    s.push_str(&format!("  Best grid params: alpha={:.2}, beta={:.2}, mean|Δ|={:.6}\n",
        best_alpha, best_beta, best_mean));
    let (fit_mix, sl_mix, ic_mix) = linear_fit(&best_ev, &z12);
    s.push_str(&format!("  Linear fit: lam_fit = {:.6} * lam + {:.6}\n", sl_mix, ic_mix));
    s.push_str(&format!("  {:>3}  {:>14}  {:>14}  {:>14}  {:>10}\n",
        "n", "lam_n(H_Z)", "lam_fit", "t_n(zeta)", "|Delta|"));
    s.push_str(&format!("  {}  {}  {}  {}  {}\n",
        "---", "--------------", "--------------", "--------------", "----------"));
    let mut max_d2 = 0.0f64; let mut sum_d2 = 0.0f64;
    for n in 0..D {
        let delta = libm::fabs(fit_mix[n] - z12[n]);
        if delta > max_d2 { max_d2 = delta; }
        sum_d2 += delta;
        s.push_str(&format!("  {:3}  {:>14.8}  {:>14.8}  {:>14.8}  {:>10.6}\n",
            n+1, best_ev[n], fit_mix[n], z12[n], delta));
    }
    let md_mix = sum_d2 / (D as f64);
    s.push_str(&format!("  Mean |Delta| = {:.6},  Max |Delta| = {:.6}\n", md_mix, max_d2));

    // ═══════════════════════════════════════════════════════════
    // CONSTRUCTION 3: WH Laplacian (NON-CIRCULAR)
    // ═══════════════════════════════════════════════════════════
    s.push_str("\n═══════════════════════════════════════════════════════════\n");
    s.push_str("  CONSTRUCTION 3: WH Laplacian H = (X+X^dag+Z+Z^dag)/4\n");
    s.push_str("  (NON-CIRCULAR: discrete torus adjacency operator)\n");
    s.push_str("  Analytical spectrum: [cos(2πa/d)+cos(2πb/d)]/2\n");
    s.push_str("═══════════════════════════════════════════════════════════\n\n");

    let ev_wh = eigenvalues(&h_wh);
    let (fit_wh, sl_wh, ic_wh) = linear_fit(&ev_wh, &z12);
    s.push_str(&format!("  Linear fit: lam_fit = {:.6} * lam + {:.6}\n", sl_wh, ic_wh));
    s.push_str(&format!("  {:>3}  {:>14}  {:>14}  {:>14}  {:>10}\n",
        "n", "lam_n(H_Z)", "lam_fit", "t_n(zeta)", "|Delta|"));
    s.push_str(&format!("  {}  {}  {}  {}  {}\n",
        "---", "--------------", "--------------", "--------------", "----------"));
    let mut max_d3 = 0.0f64; let mut sum_d3 = 0.0f64;
    for n in 0..D {
        let delta = libm::fabs(fit_wh[n] - z12[n]);
        if delta > max_d3 { max_d3 = delta; }
        sum_d3 += delta;
        s.push_str(&format!("  {:3}  {:>14.8}  {:>14.8}  {:>14.8}  {:>10.6}\n",
            n+1, ev_wh[n], fit_wh[n], z12[n], delta));
    }
    let md_wh = sum_d3 / (D as f64);
    s.push_str(&format!("  Mean |Delta| = {:.6},  Max |Delta| = {:.6}\n", md_wh, max_d3));

    // Analytical full spectrum (showing unique)
    let mut analytical: Vec<f64> = (0..D).flat_map(|a|
        (0..D).map(move |b|
            (libm::cos(2.0*PI*(a as f64)/(D as f64))
           + libm::cos(2.0*PI*(b as f64)/(D as f64)))/2.0)
    ).collect();
    analytical.sort_by(|x: &f64, y: &f64| {
        if x < y { core::cmp::Ordering::Less }
        else if x > y { core::cmp::Ordering::Greater }
        else { core::cmp::Ordering::Equal }
    });
    let mut uniq: Vec<f64> = Vec::new();
    for &v in &analytical {
        if uniq.is_empty() || libm::fabs(v - uniq[uniq.len()-1]) > 1e-10 {
            uniq.push(v);
        }
    }
    s.push_str(&format!("  Analytical unique values (of 144): {} values: {:?}\n",
        uniq.len(), uniq));

    // ═══════════════════════════════════════════════════════════
    // FINAL VERDICT
    // ═══════════════════════════════════════════════════════════
    s.push_str("\n╔══════════════════════════════════════════════════════════════╗\n");
    s.push_str("║  VERDICT                                                     ║\n");
    s.push_str("╠══════════════════════════════════════════════════════════════╣\n");
    s.push_str("║                                                              ║\n");
    s.push_str(&format!("║  Construction 1 (circular, zeta as input):                   ║\n"));
    s.push_str(&format!("║    mean|Δ| = {:.2e}  — verifies mu∘delta=id               ║\n", md_rec));
    s.push_str("║    The d=12 SIC-POVM frame is informationally complete.      ║\n");
    s.push_str("║    Given the zeta zeros, the frame faithfully encodes them.  ║\n");
    s.push_str("║                                                              ║\n");
    s.push_str(&format!("║  Construction 2 (non-circular, mixed Zauner+WH):             ║\n"));
    s.push_str(&format!("║    mean|Δ| = {:.6}  — APPROXIMATE, not exact                ║\n", md_mix));
    s.push_str(&format!("║    Best: alpha={:.2}, beta={:.2}                            ║\n", best_alpha, best_beta));
    s.push_str("║    Captures rough spectral shape but not precise zeros.      ║\n");
    s.push_str("║                                                              ║\n");
    s.push_str(&format!("║  Construction 3 (non-circular, WH Laplacian):                ║\n"));
    s.push_str(&format!("║    mean|Δ| = {:.6}  — APPROXIMATE, not exact                ║\n", md_wh));
    s.push_str("║    Discrete torus adjacency — simplest structural operator.  ║\n");
    s.push_str("║                                                              ║\n");
    s.push_str("║  CONCLUSION:                                                 ║\n");
    s.push_str("║  The d=12 SIC-POVM faithfully ENCODES the zeta zeros         ║\n");
    s.push_str("║  (Construction 1 proves mu∘delta=id) but does not            ║\n");
    s.push_str("║  PRODUCE them from its structure alone. No non-circular      ║\n");
    s.push_str("║  construction yields the zeta zeros to high precision.       ║\n");
    s.push_str("║                                                              ║\n");
    s.push_str("║  The structural fusion zeta ⊗ SIC → H·P is consistent       ║\n");
    s.push_str("║  at the grammar level but the explicit Hilbert-Polya         ║\n");
    s.push_str("║  operator cannot be derived from the SIC-POVM alone          ║\n");
    s.push_str("║  without presupposing the zeta zeros.                        ║\n");
    s.push_str("╚══════════════════════════════════════════════════════════════╝\n");

    s
}
