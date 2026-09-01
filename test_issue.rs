use std::fs;
use std::path::Path;

fn main() {
    // Read the canonical words from btclattice.txt
    let content = fs::read_to_string(Path::new("ig-docs/btclattice.txt")).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    
    // Find the canonical lines
    let mut canonical_pubkey: Option<&str> = None;
    let mut canonical_privkey: Option<&str> = None;
    
    for line in lines.iter() {
        if line.contains("Privkey") && line.contains("⊞∈≻⋈⊢⊢⊢∈≺⊢⊡⊡≻⊙⊙∈⋈⊡⊡⋈≺≻⊞⊡⊣⊥⊡∋⊣⊢⊙⊢") {
            // Extract the word from the line
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                canonical_privkey = Some(parts[1].trim());
            }
        } else if line.contains("pubkey") && line.contains("≺≻⊞≺⊡") {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                canonical_pubkey = Some(parts[1].trim());
            }
        }
    }
    
    println!("Canonical pubkey: {:?}", canonical_pubkey.unwrap_or(&"NOT FOUND"));
    println!("Canonical privkey: {:?}", canonical_privkey.unwrap_or(&"NOT FOUND"));
    
    // Now test the mapping from vox-ce MARKS
    let marks = ['⊢', '⊣', '≻', '≺', '⋈', '⊤', '∈', '∋', '⊙', '⊥', '⊞', '⊡'];
    
    // Test the canonical pubkey
    if let Some(pubkey) = canonical_pubkey {
        println!("\nCanonical pubkey length: {}", pubkey.len());
        for (i, c) in pubkey.chars().enumerate() {
            if let Some(pos) = marks.iter().position(|&m| m == c) {
                println!("  Position {}: '{}' -> byte {}", i, c, pos);
            }
        }
    }
}
