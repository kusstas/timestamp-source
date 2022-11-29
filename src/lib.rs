//! Timestamp source library provides simple traits for handling timestamps.

/// Represents a timestamp.
pub trait Timestamp {
    type Duration: PartialOrd;
    type Error;

    /// Get current timestamp.
    fn now() -> Self;

    /// Get duration since start.
    fn duration_since_epoch(self) -> Self::Duration;

    /// Get duration since `other`.
    /// # Errors
    /// This function will return an error if duration between from-to timestamps is negative.
    fn duration_since(&self, other: &Self) -> Result<Self::Duration, Self::Error>;
}

/// Represents an elapsed timer.
pub trait ElapsedTimer {
    type Timestamp: Timestamp;

    /// Returns true if a timer duration is more(equal) then duration between from-to timestamps,
    /// otherwise false.
    /// # Errors
    /// This function will return an error if duration between from-to timestamps is negative.
    fn is_timeout(
        &self,
        from: &Self::Timestamp,
        to: &Self::Timestamp,
    ) -> Result<bool, <Self::Timestamp as Timestamp>::Error>;
}
