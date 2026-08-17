//! Unit for Unix timestamp conversion.

/// Unit for Unix timestamp conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnixTimeUnit {
    /// Seconds since epoch.
    Second,
    /// Milliseconds since epoch.
    Millisecond,
}
