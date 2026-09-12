//! Native certificate manifest generation for the ABC/IUTT measurement stream.
//! The output is consumed by the Lean certificate generator; no floating-point
//! value is promoted to a proof inside this module.

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChampionEvent {
    pub cutoff: u64,
    pub previous: Option<crate::abc_iutt::Triple>,
    pub champion: crate::abc_iutt::Triple,
    pub discrepancy: f64,
    /// Trilattice state: every displacement retains the old and new reading,
    /// so the transition is carried in the kernel's Both state.
    pub state: crate::belnap::B4,
}

#[derive(Clone, Debug)]
pub struct ThetaLinkCertificate {
    pub source: String,
    pub target: String,
    pub steps: usize,
    pub gate_delta: (f64, f64, f64),
    pub state: crate::belnap::B4,
}

/// Export a Teichmüller path as a trilattice Θ-link certificate.
pub fn theta_link_certificate(source: &str, target: &str) -> Option<ThetaLinkCertificate> {
    let path = crate::iuft_teichmuller::teichmuller_path(source, target)?;
    Some(ThetaLinkCertificate {
        source: path.source,
        target: path.target,
        steps: path.steps.len(),
        gate_delta: path.gate_delta,
        state: crate::belnap::B4::B,
    })
}

pub fn champion_trace(eps: f64, max_c: u64) -> Vec<ChampionEvent> {
    let mut events = Vec::new();
    let mut current: Option<(f64, crate::abc_iutt::Triple)> = None;
    for c in 2..=max_c {
        for t in crate::abc_iutt::enumerate_height(c) {
            let value = crate::abc_iutt::discrepancy(t, eps);
            if current.map_or(true, |(best, _)| value > best) {
                events.push(ChampionEvent {
                    cutoff: c,
                    previous: current.map(|(_, old)| old),
                    champion: t,
                    discrepancy: value,
                    state: crate::belnap::B4::B,
                });
                current = Some((value, t));
            }
        }
    }
    events
}

pub fn champion_report(eps: f64, max_c: u64) -> String {
    let trace = champion_trace(eps, max_c);
    let mut out = format!("ABC champion trace — ε={eps}, c≤{max_c}\n");
    for (i, e) in trace.iter().enumerate() {
        let previous = e
            .previous
            .map(|p| format!("({}, {}, {})", p.a, p.b, p.c))
            .unwrap_or_else(|| "none".into());
        out.push_str(&format!(
            "  event {i}: c={}  {previous} → ({}, {}, {})  discrepancy={:.6}  state={}\n",
            e.cutoff,
            e.champion.a,
            e.champion.b,
            e.champion.c,
            e.discrepancy,
            e.state.name()
        ));
    }
    out.push_str(&format!("events={}\n", trace.len()));
    out
}

/// Emit Lean-generator input directly from the Rust measurement engine.
pub fn manifest(eps: f64, cutoffs: &[u64]) -> String {
    let mut out = format!("epsilon={eps:.12}\n# cutoff a b c discrepancy\n");
    for &cutoff in cutoffs {
        if let Some((value, t)) = crate::abc_iutt::window_maximum(eps, cutoff) {
            out.push_str(&format!("{cutoff} {} {} {} {value:.12}\n", t.a, t.b, t.c));
        }
    }
    out
}

/// Stable JSON artifact for callers that want the native Rust path without
/// going through the REPL's command parser.
pub fn json(eps: f64, cutoffs: &[u64]) -> String {
    crate::abc_iutt::stream_json_report(eps, cutoffs)
}

/// Machine-readable champion trace. The `state` and its two lanes are part of
/// the native certificate: a displacement carries both the prior and newly
/// selected reading, so it is represented by trilattice `B`.
pub fn champion_json_report(eps: f64, max_c: u64) -> String {
    let events = champion_trace(eps, max_c);
    let encoded: Vec<String> = events.iter().map(|e| {
        let previous = e.previous
            .map(|p| format!("{{\"a\":{},\"b\":{},\"c\":{}}}", p.a, p.b, p.c))
            .unwrap_or_else(|| "null".into());
        format!(
            "{{\"cutoff\":{},\"previous\":{},\"triple\":{{\"a\":{},\"b\":{},\"c\":{}}},\"discrepancy\":{:.12},\"state\":\"{}\",\"truth\":true,\"falsehood\":true}}",
            e.cutoff, previous, e.champion.a, e.champion.b, e.champion.c,
            e.discrepancy, e.state.name())
    }).collect();
    format!(
        "{{\"epsilon\":{eps:.12},\"max_c\":{max_c},\"events\":[{}]}}",
        encoded.join(",")
    )
}

/// GPU-backed champion trace. Enumeration and radical construction remain the
/// reference path; CUDA evaluates each chunk's observable and carries the B4
/// state back with it. Chunking bounds device memory independently of `max_c`.
#[cfg(feature = "hosted")]
pub fn gpu_champion_trace(
    eps: f64,
    max_c: u64,
    device: usize,
) -> Result<Vec<ChampionEvent>, String> {
    const CHUNK: usize = 1 << 20;
    let mut pending: Vec<(crate::abc_iutt::Triple, u64)> = Vec::new();
    let mut events = Vec::new();
    let mut current: Option<(f64, crate::abc_iutt::Triple)> = None;
    let mut consume = |batch: &mut Vec<(crate::abc_iutt::Triple, u64)>| -> Result<(), String> {
        if batch.is_empty() { return Ok(()); }
        let c: Vec<u64> = batch.iter().map(|(t, _)| t.c).collect();
        let rad: Vec<u64> = batch.iter().map(|(_, r)| *r).collect();
        let readings = crate::gpu_abc::observable_batch(&c, &rad, eps, device)?;
        for ((t, _), reading) in batch.drain(..).zip(readings) {
            let value = reading.discrepancy;
            let state = reading.state;
            if current.map_or(true, |(best, _)| value > best) {
                events.push(ChampionEvent { cutoff: t.c, previous: current.map(|(_, old)| old), champion: t, discrepancy: value, state });
                current = Some((value, t));
            }
        }
        Ok(())
    };
    for c in 2..=max_c {
        for t in crate::abc_iutt::enumerate_height(c) {
            pending.push((t, crate::abc_iutt::triple_radical(t)));
            if pending.len() == CHUNK { consume(&mut pending)?; }
        }
    }
    consume(&mut pending)?;
    Ok(events)
}

#[cfg(feature = "hosted")]
pub fn gpu_champion_report(eps: f64, max_c: u64, device: usize) -> Result<String, String> {
    let trace = gpu_champion_trace(eps, max_c, device)?;
    let mut out = format!("ABC GPU champion trace — ε={eps}, c≤{max_c}, device={device}\n");
    for (i, e) in trace.iter().enumerate() {
        let previous = e.previous.map(|p| format!("({}, {}, {})", p.a, p.b, p.c)).unwrap_or_else(|| "none".into());
        out.push_str(&format!("  event {i}: c={}  {previous} → ({}, {}, {})  discrepancy={:.6}  state={}\n", e.cutoff, e.champion.a, e.champion.b, e.champion.c, e.discrepancy, e.state.name()));
    }
    out.push_str(&format!("events={}\n", trace.len()));
    Ok(out)
}

#[cfg(feature = "hosted")]
pub fn gpu_champion_reports_many(epsilons: &[f64], max_c: u64, device: usize) -> Result<Vec<String>, String> {
    const CHUNK: usize = 1 << 20;
    let mut pending: Vec<(crate::abc_iutt::Triple, u64)> = Vec::new();
    let mut traces: Vec<Vec<ChampionEvent>> = (0..epsilons.len()).map(|_| Vec::new()).collect();
    let mut current: Vec<Option<(f64, crate::abc_iutt::Triple)>> = vec![None; epsilons.len()];
    let mut consume = |batch: &mut Vec<(crate::abc_iutt::Triple, u64)>| -> Result<(), String> {
        if batch.is_empty() { return Ok(()); }
        let c: Vec<u64> = batch.iter().map(|(t, _)| t.c).collect();
        let rad: Vec<u64> = batch.iter().map(|(_, r)| *r).collect();
        let values = crate::gpu_abc::observable_batch_many(&c, &rad, epsilons, device)?;
        for (j, vals) in values.into_iter().enumerate() {
            for ((t, _), value) in batch.iter().copied().zip(vals) {
                if current[j].map_or(true, |(best, _)| value > best) {
                    traces[j].push(ChampionEvent { cutoff: t.c, previous: current[j].map(|(_, old)| old), champion: t, discrepancy: value, state: crate::belnap::B4::B });
                    current[j] = Some((value, t));
                }
            }
        }
        batch.clear();
        Ok(())
    };
    for c in 2..=max_c {
        for t in crate::abc_iutt::enumerate_height(c) {
            pending.push((t, crate::abc_iutt::triple_radical(t)));
            if pending.len() == CHUNK { consume(&mut pending)?; }
        }
    }
    consume(&mut pending)?;
    Ok(traces.into_iter().enumerate().map(|(j, trace)| {
        let mut out = format!("ABC GPU champion trace — ε={}, c≤{}, device={}\n", epsilons[j], max_c, device);
        for (i, e) in trace.iter().enumerate() {
            let previous = e.previous.map(|p| format!("({}, {}, {})", p.a, p.b, p.c)).unwrap_or_else(|| "none".into());
            out.push_str(&format!("  event {i}: c={}  {previous} → ({}, {}, {})  discrepancy={:.6}  state={}\n", e.cutoff, e.champion.a, e.champion.b, e.champion.c, e.discrepancy, e.state.name()));
        }
        out.push_str(&format!("events={}\n", trace.len()));
        out
    }).collect())
}

#[allow(dead_code)]
fn _keep_vec_link(_: Vec<u64>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_cutoff_trace_is_trilattice_both() {
        let trace = champion_trace(0.1, 9);
        assert_eq!(trace.len(), 2);
        assert_eq!(trace[0].cutoff, 2);
        assert_eq!(trace[1].cutoff, 9);
        assert!(trace
            .iter()
            .all(|event| event.state == crate::belnap::B4::B));
    }
}
