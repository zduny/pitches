//! Interval between frequencies.

use std::fmt::Display;

use ordered_float::NotNan;

/// Interval between pitches.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    cents: Cents,
}

impl Interval {
    /// Create new interval between frequencies.
    ///
    /// Frequencies must be non-zero, otherwise may panic.
    ///
    /// Positive when `frequency_0` < `frequency_1`.<br>
    /// Negative when `frequency_0` > `frequency_1`.
    pub fn new(frequency_0: f64, frequency_1: f64) -> Self {
        let cents = 1200.0 * (frequency_1 / frequency_0).ln() / (2.0_f64).ln();
        let cents = Cents(NotNan::new(cents).unwrap());
        Interval { cents }
    }

    /// Get interval in cents.
    ///
    /// 100 cents = 1 semitone.
    pub fn cents(&self) -> Cents {
        self.cents
    }
}

/// Interval in cents.
///
/// 100 cents = 1 semitone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cents(pub NotNan<f64>);

impl Cents {
    /// Get absolute (non-negative) value.
    pub fn abs(self) -> Cents {
        Cents(NotNan::new(self.0.abs()).unwrap())
    }
}

impl Display for Cents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
