//! Structured representation of musical pitches, notes and intervals
//! for for equal-tempered scale, A₄ = 440 Hz.

mod interval;
mod note;

pub use interval::*;
pub use note::*;

use lazy_static::lazy_static;
use std::fmt::Display;

/// Frequencies of pitches for equal-tempered scale, A₄ = 440 Hz.
///
/// See [table](https://pages.mtu.edu/~suits/notefreqs.html).
pub const FREQUENCIES: [f64; 108] = [
    16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50, 29.14, 30.87, 32.70,
    34.65, 36.71, 38.89, 41.20, 43.65, 46.25, 49.00, 51.91, 55.00, 58.27, 61.74, 65.41, 69.30,
    73.42, 77.78, 82.41, 87.31, 92.50, 98.00, 103.83, 110.00, 116.54, 123.47, 130.81, 138.59,
    146.83, 155.56, 164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08, 246.94, 261.63, 277.18,
    293.66, 311.13, 329.63, 349.23, 369.99, 392.00, 415.30, 440.00, 466.16, 493.88, 523.25, 554.37,
    587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.00, 932.33, 987.77, 1046.50,
    1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22, 1760.00, 1864.66,
    1975.53, 2093.00, 2217.46, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96, 3322.44,
    3520.00, 3729.31, 3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04, 5587.65, 5919.91,
    6271.93, 6644.88, 7040.00, 7458.62, 7902.13,
];

lazy_static! {
    /// All available pitches of equal-tempered scale, A₄ = 440 Hz.
    pub static ref PITCHES: Vec<Pitch> = (0..FREQUENCIES.len())
        .into_iter()
        .map(|index| Pitch { index: index as u8 })
        .collect();
}

/// Struct representing pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch {
    index: u8,
}

impl Pitch {
    /// Frequency of pitch.
    pub fn frequency(&self) -> f64 {
        FREQUENCIES[self.index as usize]
    }

    /// Get index in [FREQUENCIES] array.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Get 'number' of pitch.
    ///
    ///| Number | Note  |
    ///|--------|-------|
    ///| 0      | C     |
    ///| 1      | C♯/D♭ |
    ///| 2      | D     |
    ///| 3      | D♯/E♭ |
    ///| 4      | E     |
    ///| 5      | F     |
    ///| 6      | F♯/G♭ |
    ///| 7      | G     |
    ///| 8      | G♯/A♭ |
    ///| 9      | A     |
    ///| 10     | A♯/B♭ |
    ///| 11     | B     |
    pub fn number(&self) -> u8 {
        self.index % 12
    }

    /// Octave of the pitch.
    pub fn octave(&self) -> Octave {
        (self.index / 12).try_into().unwrap()
    }
}

impl Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FREQUENCIES[self.index as usize])
    }
}
