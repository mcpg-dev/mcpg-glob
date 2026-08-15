//! Shared glob pattern matching for tool name filtering.
//!
//! Supports:
//! - `*` — matches any sequence of characters
//! - `?` — matches exactly one character
//! - Literal characters match themselves
//!
//! Extracted into one shared implementation from copies that had been
//! duplicated across several plugin crates.

/// Match a glob pattern against a text string.
///
/// - `*` matches any sequence of characters (greedy)
/// - `?` matches exactly one character
/// - All other characters match literally
///
/// Linear-time, non-recursive matcher: it walks both strings with a
/// single pass and remembers the position of the most recent `*`
/// (`star_pi`) plus how far the text had advanced when that `*` was
/// seen (`star_ti`). On a literal mismatch it backtracks to that `*`
/// and lets it consume one more text byte, which avoids the
/// exponential blow-up a naive recursive matcher hits on patterns
/// with many wildcards.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let p = pattern.as_bytes();
    let t = text.as_bytes();

    let mut pi = 0usize;
    let mut ti = 0usize;
    // Last `*` seen in the pattern and the text offset at that point;
    // the backtrack target when a later literal fails to match.
    let mut star_pi: Option<usize> = None;
    let mut star_ti = 0usize;

    while ti < t.len() {
        if pi < p.len() && (p[pi] == b'?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == b'*' {
            star_pi = Some(pi);
            star_ti = ti;
            pi += 1;
        } else if let Some(sp) = star_pi {
            pi = sp + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }

    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }

    pi == p.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_match() {
        assert!(glob_match("hello", "hello"));
        assert!(!glob_match("hello", "world"));
    }

    #[test]
    fn star_matches_any_sequence() {
        assert!(glob_match("orders.*", "orders.place_order"));
        assert!(glob_match("orders.*", "orders.list"));
        assert!(!glob_match("orders.*", "finance.transfer"));
    }

    #[test]
    fn star_matches_across_dots() {
        assert!(glob_match("*", "orders.place_order"));
        assert!(glob_match("*", "a.b.c.d"));
    }

    #[test]
    fn question_mark_matches_single_char() {
        assert!(glob_match("test?", "test1"));
        assert!(glob_match("test?", "testA"));
        assert!(!glob_match("test?", "test"));
        assert!(!glob_match("test?", "test12"));
    }

    #[test]
    fn complex_patterns() {
        assert!(glob_match("*.place_*", "orders.place_order"));
        assert!(glob_match("*.place_*", "finance.place_trade"));
        assert!(!glob_match("*.place_*", "orders.list"));
    }

    #[test]
    fn empty_pattern() {
        assert!(glob_match("", ""));
        assert!(!glob_match("", "hello"));
    }

    #[test]
    fn star_at_end() {
        assert!(glob_match("prefix*", "prefix_anything"));
        assert!(glob_match("prefix*", "prefix"));
    }

    #[test]
    fn pattern_debug_prefix() {
        assert!(glob_match("orders.debug_*", "orders.debug_dump"));
        assert!(!glob_match("orders.debug_*", "orders.place_order"));
    }

    #[test]
    fn mcpg_wildcard_exclude() {
        assert!(glob_match("mcpg.*", "mcpg.runtime_snapshot"));
        assert!(!glob_match("mcpg.*", "orders.place_order"));
    }
}
