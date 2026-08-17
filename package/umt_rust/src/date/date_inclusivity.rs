//! Inclusivity for date range comparisons.

/// Inclusivity for range comparisons.
///
/// Matches the TypeScript `DateInclusivity` strings:
/// `()` exclusive, `[]` inclusive, `[)` start-inclusive, `(]` end-inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateInclusivity {
    /// Exclusive start and end (`()`).
    Exclusive,
    /// Inclusive start and end (`[]`).
    Inclusive,
    /// Inclusive start, exclusive end (`[)`).
    StartInclusive,
    /// Exclusive start, inclusive end (`(]`).
    EndInclusive,
}

impl DateInclusivity {
    /// Returns `true` when the start bound is inclusive.
    pub const fn include_start(self) -> bool {
        matches!(self, Self::Inclusive | Self::StartInclusive)
    }

    /// Returns `true` when the end bound is inclusive.
    pub const fn include_end(self) -> bool {
        matches!(self, Self::Inclusive | Self::EndInclusive)
    }
}
