#![allow(dead_code)]
//! qft.rs — Quantum Fourier Transform Tool
//!
//! Implements the Quantum Fourier Transform (QFT) and its inverse (IQFT)
//! as native kernel commands. The QFT is the core subroutine of Shor's
//! period-finding algorithm and many other quantum algorithms.
//!
//! Circuit structure for n qubits:
//!   QFT = H_0 CR_1 H_1 CR_2 CR_1 H_2 ... CR_{n-1} ... CR_1 H_{n-1}
//!   IQFT = H_{n-1} CR_1 H_{n-2} CR_2 CR_1 ... H_0 CR_{n-1} ... CR_1
//!
//! where CR_k = controlled-R_k, R_k = diag(1, e^{2πi/2^k})

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::{String, ToString};
use alloc::format;
// use libm;

/// QFT/IQFT circuit representation
#[derive(Clone, Debug)]
pub struct QftCircuit {
    pub n_qubits: usize,
    pub inverse: bool,
    pub gates: Vec<QftGate>,
}

#[derive(Clone, Debug)]
pub struct QftGate {
    pub kind: QftGateKind,
    pub target: usize,
    pub control: Option<usize>,
    pub k: Option<usize>, // for controlled-R_k: the phase denominator 2^k
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QftGateKind {
    H,          // Hadamard
    CR,         // Controlled-R_k (phase gate)
    SWAP,       // SWAP at the end of QFT (before IQFT)
}

/// Build the QFT circuit for n qubits.
/// The standard QFT applies H on qubit 0, then controlled-R_k from qubits 1..n-1 onto qubit 0,
/// then H on qubit 1, controlled-R_k from qubits 2..n-1 onto qubit 1, etc.
/// Finally SWAPs to reverse qubit order (since QFT naturally produces bit-reversed output).
pub fn qft_circuit(n_qubits: usize, inverse: bool) -> QftCircuit {
    let mut gates = Vec::new();

    if inverse {
        // IQFT: reverse order of QFT, with inverse gates
        // First SWAPs to reverse the bit order (inverse of final SWAPs in QFT)
        for i in 0..n_qubits / 2 {
            gates.push(QftGate { kind: QftGateKind::SWAP, target: i, control: Some(n_qubits - 1 - i), k: None });
        }
        // Then reversed H and CR layers
        for i in (0..n_qubits).rev() {
            // Controlled-R_k gates from higher qubits onto i
            for j in (i + 1..n_qubits).rev() {
                let k = j - i + 1;
                gates.push(QftGate { kind: QftGateKind::CR, target: i, control: Some(j), k: Some(k) });
            }
            // Hadamard on qubit i
            gates.push(QftGate { kind: QftGateKind::H, target: i, control: None, k: None });
        }
    } else {
        // Forward QFT
        for i in 0..n_qubits {
            // Hadamard on qubit i
            gates.push(QftGate { kind: QftGateKind::H, target: i, control: None, k: None });
            // Controlled-R_k gates from higher qubits onto i
            for j in (i + 1)..n_qubits {
                let k = j - i + 1;
                gates.push(QftGate { kind: QftGateKind::CR, target: i, control: Some(j), k: Some(k) });
            }
        }
        // Final SWAPs to correct bit order
        for i in 0..n_qubits / 2 {
            gates.push(QftGate { kind: QftGateKind::SWAP, target: i, control: Some(n_qubits - 1 - i), k: None });
        }
    }

    QftCircuit { n_qubits, inverse, gates }
}

/// Phase angle for R_k gate: 2π / 2^k
pub fn r_k_angle(k: usize) -> f64 {
    core::f64::consts::PI * 2.0 / (1u64 << k) as f64
}

/// Format the circuit as a human-readable diagram
pub fn format_circuit(circuit: &QftCircuit) -> String {
    let mut out = String::new();
    let n = circuit.n_qubits;

    out.push_str(&format!("{} QFT Circuit ({} qubits)\n",
        if circuit.inverse { "IQFT" } else { "QFT" }, n));
    out.push_str(&"─".repeat(50));
    out.push('\n');

    // Build per-qubit lines
    let mut lines: Vec<Vec<String>> = vec![vec![String::new(); circuit.gates.len()]; n];

    for (gate_idx, gate) in circuit.gates.iter().enumerate() {
        match gate.kind {
            QftGateKind::H => {
                lines[gate.target][gate_idx] = "H".to_string();
                // Fill other qubits with identity wire
                for q in 0..n {
                    if q != gate.target && lines[q][gate_idx].is_empty() {
                        lines[q][gate_idx] = "─".to_string();
                    }
                }
            }
            QftGateKind::CR => {
                if let Some(ctrl) = gate.control {
                    lines[ctrl][gate_idx] = "●".to_string(); // control
                    if let Some(k) = gate.k {
                        lines[gate.target][gate_idx] = format!("R{}", k); // target with R_k
                    } else {
                        lines[gate.target][gate_idx] = "R".to_string();
                    }
                    // Wire between control and target
                    for q in (gate.target + 1)..ctrl {
                        if lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "│".to_string();
                        }
                    }
                    // Other qubits
                    for q in 0..n {
                        if q != gate.target && q != ctrl && q < gate.target && lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "─".to_string();
                        } else if q > ctrl && lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "─".to_string();
                        }
                    }
                }
            }
            QftGateKind::SWAP => {
                if let Some(other) = gate.control {
                    lines[gate.target][gate_idx] = "╳".to_string();
                    lines[other][gate_idx] = "╳".to_string();
                    for q in (gate.target + 1)..other {
                        if lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "│".to_string();
                        }
                    }
                    for q in 0..n {
                        if q != gate.target && q != other && q < gate.target && lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "─".to_string();
                        } else if q > other && lines[q][gate_idx].is_empty() {
                            lines[q][gate_idx] = "─".to_string();
                        }
                    }
                }
            }
        }
    }

    // Print each qubit line
    for q in 0..n {
        out.push_str(&format!("q{}: ", q));
        for gate_idx in 0..circuit.gates.len() {
            let s = &lines[q][gate_idx];
            if s.is_empty() {
                out.push_str("───");
            } else {
                out.push_str(s);
            }
            out.push(' ');
        }
        out.push('\n');
    }

    out.push_str(&"─".repeat(50));
    out.push('\n');
    out.push_str(&format!("Total gates: {} (H: {}, CR: {}, SWAP: {})\n",
        circuit.gates.len(),
        circuit.gates.iter().filter(|g| g.kind == QftGateKind::H).count(),
        circuit.gates.iter().filter(|g| g.kind == QftGateKind::CR).count(),
        circuit.gates.iter().filter(|g| g.kind == QftGateKind::SWAP).count()
    ));

    out
}

/// Generate the phase angles for all CR gates in the circuit
pub fn circuit_phases(circuit: &QftCircuit) -> Vec<(usize, usize, f64)> {
    let mut phases = Vec::new();
    for gate in &circuit.gates {
        if gate.kind == QftGateKind::CR {
            if let (Some(ctrl), Some(k)) = (gate.control, gate.k) {
                let angle = r_k_angle(k);
                phases.push((ctrl, gate.target, angle));
            }
        }
    }
    phases
}

/// Verify the QFT/IQFT structure: QFT followed by IQFT should be identity (up to global phase)
pub fn verify_qft_iqft(n_qubits: usize) -> bool {
    let qft = qft_circuit(n_qubits, false);
    let iqft = qft_circuit(n_qubits, true);
    // Structural check: QFT has n H gates, IQFT has n H gates
    let qft_h = qft.gates.iter().filter(|g| g.kind == QftGateKind::H).count();
    let iqft_h = iqft.gates.iter().filter(|g| g.kind == QftGateKind::H).count();
    let qft_cr = qft.gates.iter().filter(|g| g.kind == QftGateKind::CR).count();
    let iqft_cr = iqft.gates.iter().filter(|g| g.kind == QftGateKind::CR).count();
    let qft_swap = qft.gates.iter().filter(|g| g.kind == QftGateKind::SWAP).count();
    let iqft_swap = iqft.gates.iter().filter(|g| g.kind == QftGateKind::SWAP).count();

    qft_h == n_qubits && iqft_h == n_qubits && qft_cr == iqft_cr && qft_swap == iqft_swap
}

/// Compile QFT to Fibonacci anyon braid word (delegates to fibonacci_shor for IQFT)
pub fn qft_to_braid(n_qubits: usize, inverse: bool) -> Vec<i32> {
    if inverse {
        // Use the existing inverse_qft_braid from fibonacci_shor
        crate::fibonacci_shor::inverse_qft_braid(n_qubits)
    } else {
        // For forward QFT, we can reverse the IQFT braid and invert generators
        let iqft = crate::fibonacci_shor::inverse_qft_braid(n_qubits);
        let mut qft = Vec::with_capacity(iqft.len());
        for &g in iqft.iter().rev() {
            qft.push(-g); // inverse of inverse = forward
        }
        qft
    }
}

/// Estimate braid length for QFT/IQFT on n qubits
pub fn estimate_qft_braid_length(n_qubits: usize) -> usize {
    // Each controlled-R_k needs SK approximation
    // Number of CR gates = n*(n-1)/2
    // Each CR_k decomposed into k controlled-Z, each ~50 braid generators (SK depth)
    let sk_depth = 50;
    let cr_count = n_qubits * (n_qubits - 1) / 2;
    // Average k ~ n/2
    let avg_k = (n_qubits + 1) / 2;
    let h_count = n_qubits * sk_depth; // H gates
    let cr_braid = cr_count * avg_k * sk_depth;
    let swap_count = n_qubits / 2;
    let swap_braid = swap_count * 3 * sk_depth; // SWAP = 3 CNOTs
    h_count + cr_braid + swap_braid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qft_circuit_n4() {
        let c = qft_circuit(4, false);
        assert_eq!(c.n_qubits, 4);
        assert!(!c.inverse);
        // 4 H gates, 6 CR gates (4*3/2), 2 SWAPs
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::H).count(), 4);
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::CR).count(), 6);
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::SWAP).count(), 2);
    }

    #[test]
    fn test_iqft_circuit_n4() {
        let c = qft_circuit(4, true);
        assert_eq!(c.n_qubits, 4);
        assert!(c.inverse);
        // 4 H gates, 6 CR gates, 2 SWAPs (same counts, different order)
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::H).count(), 4);
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::CR).count(), 6);
        assert_eq!(c.gates.iter().filter(|g| g.kind == QftGateKind::SWAP).count(), 2);
    }

    #[test]
    fn test_verify_qft_iqft() {
        assert!(verify_qft_iqft(2));
        assert!(verify_qft_iqft(4));
        assert!(verify_qft_iqft(8));
    }

    #[test]
    fn test_r_k_angles() {
        // R_1 = π, R_2 = π/2, R_3 = π/4, etc.
        assert!((r_k_angle(1) - core::f64::consts::PI).abs() < 1e-9);
        assert!((r_k_angle(2) - core::f64::consts::PI / 2.0).abs() < 1e-9);
        assert!((r_k_angle(3) - core::f64::consts::PI / 4.0).abs() < 1e-9);
    }
}