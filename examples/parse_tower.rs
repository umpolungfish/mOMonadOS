// parse_tower.rs — Parse PARI tower polynomials and generate Rust source with U256 coefficients
// Run with: cargo run --example parse_tower

use std::fs;

fn main() {
    let base_path = "/home/mrnob0dy666/imsgct/d12_sic_build";
    
    // Parse all tower polynomials
    let c1 = parse_poly(&format!("{}/tower_C1.poly", base_path));
    let c4 = parse_poly(&format!("{}/tower_C4.poly", base_path));
    let c8 = parse_poly(&format!("{}/tower_C8.poly", base_path));
    let c16 = parse_poly(&format!("{}/tower_C16.poly", base_path));
    let c32 = parse_poly(&format!("{}/tower_C32.poly", base_path));
    
    // Generate Rust source
    generate_rust_source(&c1, &c4, &c8, &c16, &c32);
}

fn parse_poly(path: &str) -> Vec<u64> {
    let content = fs::read_to_string(path).expect(&format!("Failed to read {}", path));
    let poly_str = content.trim();
    
    println!("Parsing {} (length: {})", path, poly_str.len());
    
    // Parse PARI format: x^n + a_{n-1}x^{n-1} + ... + a_0
    // or with negative terms
    let terms = split_terms(poly_str);
    let mut coeffs = std::collections::BTreeMap::new();
    let mut max_deg = 0;
    
    for (sign, term) in terms {
        if term.starts_with('x') {
            let deg = if term.len() > 1 && term.chars().nth(1) == Some('^') {
                term[2..].parse::<u32>().unwrap_or(1)
            } else {
                1
            };
            coeffs.insert(deg, sign * 1);
            if deg > max_deg { max_deg = deg; }
        } else if term.contains('x') {
            let parts: Vec<&str> = term.split('x').collect();
            let coeff: i128 = parts[0].parse().unwrap_or(1);
            let deg = if parts.len() > 1 && parts[1].starts_with('^') {
                parts[1][1..].parse::<u32>().unwrap_or(1)
            } else if parts.len() > 1 {
                1
            } else {
                0
            };
            coeffs.insert(deg, sign * coeff);
            if deg > max_deg { max_deg = deg; }
        } else {
            let coeff: i128 = term.parse().unwrap_or(0);
            coeffs.insert(0, sign * coeff);
        }
    }
    
    // Convert to u64 limbs (U256 = 4×u64 little-endian)
    let mut result = vec![0u64; 4 * (max_deg as usize + 1)];
    
    for (deg, coeff) in coeffs {
        if coeff == 0 { continue; }
        let abs_coeff = coeff.unsigned_abs() as u128;
        let is_negative = coeff < 0;
        
        // Convert to 4×u64 limbs (128-bit value fits in 2×u64)
        let low = abs_coeff as u64;
        let high = (abs_coeff >> 64) as u64;
        
        let base = deg as usize * 4;
        if is_negative {
            // Store as two's complement
            result[base] = (!low).wrapping_add(1);
            result[base + 1] = (!high).wrapping_add(1);
            result[base + 2] = u64::MAX;
            result[base + 3] = u64::MAX;
        } else {
            result[base] = low;
            result[base + 1] = high;
            result[base + 2] = 0;
            result[base + 3] = 0;
        }
    }
    
    // Ensure monic (leading coefficient = 1)
    let leading_base = max_deg as usize * 4;
    result[leading_base] = 1;
    result[leading_base + 1] = 0;
    result[leading_base + 2] = 0;
    result[leading_base + 3] = 0;
    
    // Flatten to coeff array
    result
}

fn split_terms(poly: &str) -> Vec<(i128, String)> {
    let mut terms = Vec::new();
    let mut current = String::new();
    let mut sign = 1i128;
    
    for ch in poly.chars() {
        match ch {
            '+' => {
                if !current.is_empty() {
                    terms.push((sign, current.trim().to_string()));
                    current.clear();
                }
                sign = 1;
            }
            '-' => {
                if !current.is_empty() {
                    terms.push((sign, current.trim().to_string()));
                    current.clear();
                }
                sign = -1;
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        terms.push((sign, current.trim().to_string()));
    }
    terms
}

fn generate_rust_source(c1: &[u64], c4: &[u64], c8: &[u64], c16: &[u64], c32: &[u64]) {
    let mut output = String::new();
    output.push_str("//! tower_polynomials.rs — PARI Tower Polynomials (U256 coefficients)\n");
    output.push_str("//! Auto-generated from tower_C*.poly files\n");
    output.push_str("#![allow(dead_code)]\n\n");
    output.push_str("use crate::pk2sk::U256;\n\n");
    
    // C1
    output.push_str(&format!("pub const TOWER_C1_DEG: u32 = {};\n", c1.len() / 4 - 1));
    output.push_str(&format!("pub const TOWER_C1_COEFFS: &[U256] = &[\n"));
    for chunk in c1.chunks(4) {
        output.push_str(&format!("    U256([{:#x}, {:#x}, {:#x}, {:#x}]),\n", 
            chunk[0], chunk[1], chunk[2], chunk[3]));
    }
    output.push_str("];\n\n");
    
    // C4
    output.push_str(&format!("pub const TOWER_C4_DEG: u32 = {};\n", c4.len() / 4 - 1));
    output.push_str(&format!("pub const TOWER_C4_COEFFS: &[U256] = &[\n"));
    for chunk in c4.chunks(4) {
        output.push_str(&format!("    U256([{:#x}, {:#x}, {:#x}, {:#x}]),\n", 
            chunk[0], chunk[1], chunk[2], chunk[3]));
    }
    output.push_str("];\n\n");
    
    // C8
    output.push_str(&format!("pub const TOWER_C8_DEG: u32 = {};\n", c8.len() / 4 - 1));
    output.push_str(&format!("pub const TOWER_C8_COEFFS: &[U256] = &[\n"));
    for chunk in c8.chunks(4) {
        output.push_str(&format!("    U256([{:#x}, {:#x}, {:#x}, {:#x}]),\n", 
            chunk[0], chunk[1], chunk[2], chunk[3]));
    }
    output.push_str("];\n\n");
    
    // C16 (MAIN FIDUCIAL)
    output.push_str(&format!("pub const TOWER_C16_DEG: u32 = {};\n", c16.len() / 4 - 1));
    output.push_str(&format!("pub const TOWER_C16_COEFFS: &[U256] = &[\n"));
    for chunk in c16.chunks(4) {
        output.push_str(&format!("    U256([{:#x}, {:#x}, {:#x}, {:#x}]),\n", 
            chunk[0], chunk[1], chunk[2], chunk[3]));
    }
    output.push_str("];\n\n");
    
    // C32
    output.push_str(&format!("pub const TOWER_C32_DEG: u32 = {};\n", c32.len() / 4 - 1));
    output.push_str(&format!("pub const TOWER_C32_COEFFS: &[U256] = &[\n"));
    for chunk in c32.chunks(4) {
        output.push_str(&format!("    U256([{:#x}, {:#x}, {:#x}, {:#x}]),\n", 
            chunk[0], chunk[1], chunk[2], chunk[3]));
    }
    output.push_str("];\n\n");
    
    // Discriminant exponents and field degrees
    output.push_str("pub const TOWER_C16_DISC_EXP_F: u32 = 32;\n");
    output.push_str("pub const TOWER_C16_DEG_Q: u32 = 64;\n");
    output.push_str("pub const TOWER_C16_DEG_F: u32 = 32;\n\n");
    output.push_str("pub const TOWER_C32_DISC_EXP_F: u32 = 64;\n");
    output.push_str("pub const TOWER_C32_DEG_Q: u32 = 128;\n");
    output.push_str("pub const TOWER_C32_DEG_F: u32 = 64;\n");
    
    fs::write("/home/mrnob0dy666/imsgct/mOMonadOS/src/tower_polynomials.rs", output)
        .expect("Failed to write tower_polynomials.rs");
    println!("Generated tower_polynomials.rs");
}