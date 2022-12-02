//! Timestamp source library provides simple traits for handling timestamps.

#![no_std]

use core::marker::PhantomData;
use spin::relax::{RelaxStrategy, Spin};

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
    /// This function will return an error if duration between other-self timestamps is negative.
    fn duration_since(&self, other: &Self) -> Result<Self::Duration, Self::Error>;
}

/// Represents an elapsed timer.
pub trait ElapsedTimer {
    type Timestamp: Timestamp;

    /// Returns true if a timer duration is more(equal) then duration between from-to timestamps,
    /// otherwise false.
    /// # Errors
    /// This function will return an error if duration between from-to timestamps is negative.
    fn timeout(
        &self,
        from: &Self::Timestamp,
        to: &Self::Timestamp,
    ) -> Result<bool, <Self::Timestamp as Timestamp>::Error>;
}

/// Implementation of [`ElapsedTimer`](crate::ElapsedTimer)
pub struct Timer<T: Timestamp> {
    duration: T::Duration,
}

impl<T: Timestamp> Timer<T> {
    /// Creates a new [`Timer<T>`].
    pub const fn new(duration: T::Duration) -> Self {
        Timer { duration }
    }

    /// Borrow the duration of this [`Timer<T>`].
    pub fn borrow_duration(&self) -> &T::Duration {
        &self.duration
    }

    /// Borrow the mutable duration of this [`Timer<T>`].
    pub fn borrow_mut_duration(&mut self) -> &mut T::Duration {
        &mut self.duration
    }
}

impl<T: Timestamp> ElapsedTimer for Timer<T> {
    type Timestamp = T;

    fn timeout(
        &self,
        from: &Self::Timestamp,
        to: &Self::Timestamp,
    ) -> Result<bool, <Self::Timestamp as Timestamp>::Error> {
        Ok(to.duration_since(from)? >= self.duration)
    }
}

/// Object that can delay by some duration.
///
/// `T` - [`Timestamp`] that provides `now`.
///
/// `R` - [`RelaxStrategy`] that provides a strategy to handle an idle.
pub struct Delay<T: Timestamp, R: RelaxStrategy = Spin> {
    duration: T::Duration,
    relax: PhantomData<R>,
}

impl<T: Timestamp, R: RelaxStrategy> Delay<T, R> {
    /// Creates a new [`Delay<T, R>`].
    pub const fn new(duration: T::Duration) -> Self {
        Delay {
            duration,
            relax: PhantomData::<R>,
        }
    }

    /// Borrow the duration of this [`Delay<T, R>`].
    pub fn borrow_duration(&self) -> &T::Duration {
        &self.duration
    }

    /// Borrow the mutable duration of this [`Delay<T, R>`].
    pub fn borrow_mut_duration(&mut self) -> &mut T::Duration {
        &mut self.duration
    }

    /// Execute delay.
    /// # Errors
    /// This function will return an error if [`Timestamp::duration_since`] returns an error.
    pub fn exec(&self) -> Result<(), T::Error> {
        let start = T::now();

        while T::now().duration_since(&start)? < self.duration {
            R::relax();
        }

        Ok(())
    }
}
