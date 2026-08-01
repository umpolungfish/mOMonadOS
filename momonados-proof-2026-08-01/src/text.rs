// String handling for a kernel whose strings are mostly not ASCII.
//
// The names in this kernel are Shavian glyphs, IG primitives and arrows. A byte
// index into any of those lands mid-character about two times in three, and
// `&s[..n]` panics when it does — which is how `triple` died on the byte 45 that
// sits inside the '→' of "bulk→boundary→reconstruction". Every truncation here
// counts characters, so the panic has nowhere to come from and the column widths
// come out right as well: `{:>8}` counts characters too, and a byte-length clip
// mis-aligns every table row containing a glyph.

/// The first `max` characters. Never splits a character, and returns the whole
/// string when it is already short enough.
pub fn clip(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

/// The first `max` characters, with an ellipsis when anything was dropped, so a
/// truncated reading is not mistaken for a complete one.
pub fn clip_ellipsis(s: &str, max: usize) -> alloc::string::String {
    use alloc::string::ToString;
    if s.chars().count() <= max { return s.to_string(); }
    let mut out = clip(s, max.saturating_sub(1)).to_string();
    out.push('…');
    out
}
