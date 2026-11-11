//! Static build info for Freemind Desktop.

pub const BUILD_VERSION: &str  = "v4.4.27";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "9f7c1bb5e34c";
pub const BUILD_TIMESTAMP: u64 = 1702163721;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Freemind Desktop", BUILD_VERSION, BUILD_CHANNEL)
}
