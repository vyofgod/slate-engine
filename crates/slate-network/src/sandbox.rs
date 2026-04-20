//! Lightweight origin allow-list. Real sandboxing (process-level)
//! is out of scope for this crate — Centrion owns that. What we do
//! here is make it *impossible* to issue a fetch that hasn't been
//! explicitly permitted by the caller.

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum OriginPolicy {
    /// Permit every origin. Intended for local dev only.
    AllowAll,
    /// Permit only the listed origins (scheme + host + optional
    /// port, lowercased). Everything else fails fast.
    AllowList(HashSet<String>),
}

impl OriginPolicy {
    pub fn allow_list<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        OriginPolicy::AllowList(items.into_iter().map(|s| s.into().to_ascii_lowercase()).collect())
    }

    pub fn permits(&self, url: &str) -> bool {
        match self {
            OriginPolicy::AllowAll => true,
            OriginPolicy::AllowList(set) => origin_of(url)
                .map(|o| set.contains(&o))
                .unwrap_or(false),
        }
    }
}

fn origin_of(url: &str) -> Option<String> {
    // "scheme://host[:port]/..."
    let scheme_end = url.find("://")?;
    let scheme = &url[..scheme_end];
    let rest = &url[scheme_end + 3..];
    let host_end = rest.find('/').unwrap_or(rest.len());
    let host = &rest[..host_end];
    Some(format!("{}://{}", scheme.to_ascii_lowercase(), host.to_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_list_rejects_cross_origin() {
        let p = OriginPolicy::allow_list(["https://a.example"]);
        assert!(p.permits("https://a.example/page.html"));
        assert!(!p.permits("https://b.example/page.html"));
        assert!(!p.permits("http://a.example/page.html")); // scheme differs
    }
}
