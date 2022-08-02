use std::fmt::Display;

use ordered_float::NotNan;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    cents: Cents,
}

impl Interval {
    pub fn new(frequency_0: f64, frequency_1: f64) -> Self {
        let cents = 1200.0 * (frequency_1 / frequency_0).ln() / (2.0_f64).ln();
        let cents = Cents(NotNan::new(cents).unwrap());
        Interval { cents }
    }

    pub fn cents(&self) -> Cents {
        self.cents
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cents(pub NotNan<f64>);

impl Cents {
    pub fn abs(self) -> Cents {
        Cents(NotNan::new(self.0.abs()).unwrap())
    }
}

impl Display for Cents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
