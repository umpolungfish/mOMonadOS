#!/usr/bin/env python3
"""Generate vault.rs from ob3ect/digital/.vault directory listing."""
import subprocess, json

VAULT_DIR = "/home/mrnob0dy666/imsgct/ob3ect/digital/.vault"

# Get all vault entries
result = subprocess.run(["find", VAULT_DIR, "-maxdepth", "1", "-type", "d"],
                       capture_output=True, text=True)
dirs = sorted([d.split("/")[-1] for d in result.stdout.strip().split("\n") if d and not d.endswith(".vault") and d != "__pycache__"])

# Load domain info
domain_entries = {}
for d in dirs:
    try:
        info_path = f"{VAULT_DIR}/{d}/_ob3ect.json"
        with open(info_path) as f:
            info = json.load(f)
            domain_entries[d] = info.get("domain", "symbolic")
    except:
        domain_entries[d] = "symbolic"

# Generate Rust file
rust_lines = []
rust_lines.append("""// vault.rs — Ob3ect Vault Registry (281 entries)
// Dynamically registered from ob3ect/digital/.vault/
// Ported to Rust for mOMonadOS — Phase 10 dynamic registry
// Author: Lando⊗⊙perator
#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::format;

use crate::belnap::B4;

// ─── Vault Entry ──────────────────────────────────────────────────

#[derive(Clone)]
pub struct VaultEntry {
    pub name: String,
    pub domain: &'static str,
    pub has_py: bool,
    pub has_lean: bool,
    pub json_path: String,
}

// ─── Vault Bootstrap — generated from .vault/ directory ──────────
""")

rust_lines.append(f"pub static VAULT_BOOTSTRAP: &[(&str, &str)] = &[")
for d in dirs:
    safe_name = d.replace("'", "\\'")
    domain = domain_entries.get(d, "symbolic")
    rust_lines.append(f'    ("{safe_name}", "{domain}"),')
rust_lines.append("];")

rust_lines.append(f"""
/// Runtime vault registry — initialized from VAULT_BOOTSTRAP on first access.
static mut VAULT_REGISTRY: Option<Vec<VaultEntry>> = None;

fn ensure_vault() -> &'static mut Vec<VaultEntry> {{
    unsafe {{
        let ptr = core::ptr::addr_of_mut!(VAULT_REGISTRY);
        if (*ptr).is_none() {{
            let mut v = Vec::new();
            for &(name, domain) in VAULT_BOOTSTRAP {{
                v.push(VaultEntry {{
                    name: String::from(name),
                    domain,
                    has_py: false,
                    has_lean: false,
                    json_path: format!("/home/mrnob0dy666/imsgct/ob3ect/digital/.vault/{{}}/_ob3ect.json", name),
                }});
            }}
            *ptr = Some(v);
        }}
        (*ptr).as_mut().unwrap()
    }}
}}

/// Register a vault entry at runtime.
pub fn register_vault_entry(name: &str, domain: &'static str) -> bool {{
    let reg = ensure_vault();
    if reg.iter().any(|e| e.name == name) {{
        return false;
    }}
    reg.push(VaultEntry {{
        name: String::from(name),
        domain,
        has_py: false,
        has_lean: false,
        json_path: String::new(),
    }});
    true
}}

/// List vault ob3ects, optionally filtered by domain.
pub fn list_vault_ob3ects(filter_domain: Option<&str>) -> String {{
    let reg = ensure_vault();
    let filtered: Vec<&VaultEntry> = if let Some(d) = filter_domain {{
        reg.iter().filter(|e| e.domain == d).collect()
    }} else {{
        reg.iter().collect()
    }};
    let mut out = format!("Vault Ob3ects ({{}}):\\n", filtered.len());
    for e in &filtered {{
        out.push_str(&format!("  {{:50}} [{{}}]\\n", e.name, e.domain));
    }}
    out
}}

/// Look up a vault entry by name.
pub fn find_vault_entry(name: &str) -> Option<VaultEntry> {{
    let reg = ensure_vault();
    reg.iter().find(|e| e.name == name).cloned()
}}

/// Domain counts summary.
pub fn vault_domain_summary() -> String {{
    let reg = ensure_vault();
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for e in reg.iter() {{
        *counts.entry(e.domain).or_insert(0) += 1;
    }}
    let mut out = String::from("Vault Domain Summary:\\n");
    for (domain, count) in &counts {{
        out.push_str(&format!("  {{:30}}: {{}}\\n", domain, count));
    }}
    out.push_str(&format!("  TOTAL: {{}}\\n", reg.len()));
    out
}}

/// Run a vault ob3ect bootstrap (stub — dispatches to domain-specific bootstrapper).
pub fn run_vault_ob3ect(name: &str) -> String {{
    let reg = ensure_vault();
    if let Some(entry) = reg.iter().find(|e| e.name == name) {{
        format!(
            "== {{}} ==\\n  Domain:     {{}}\\n  Has .py:    {{}}\\n  Has .lean:  {{}}\\n  Status:     VOID (stub)\\n  Frobenius:  N/A\\n  Output:     Vault ob3ect '{{}}' — stub (full bootstrap requires ob3ect/digital loader)\\n",
            entry.name, entry.domain, entry.has_py, entry.has_lean, entry.name
        )
    }} else {{
        format!("Unknown vault ob3ect: '{{}}'. Use 'cr3 --list-ob3ects'.", name)
    }}
}}
""")

with open("/home/mrnob0dy666/imsgct/mOMonadOS/src/cr3echrz/vault.rs", "w") as f:
    f.write("\n".join(rust_lines))

print(f"Generated vault.rs with {len(dirs)} entries")
for d in dirs[:5]:
    print(f"  {d} -> {domain_entries.get(d, 'symbolic')}")
print(f"  ... and {len(dirs)-5} more")
