//! Note representation.

use std::fmt::Display;

use crate::{Pitch, PITCHES};

/// Error that can occur during note creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Incorrect note letter.
    IncorrectLetter,
    /// Incorrect note accidental.
    IncorrectAccidental,
    /// Octave is not in supported range.
    OctaveNotInRange,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IncorrectLetter => write!(f, "incorrect letter"),
            Error::IncorrectAccidental => write!(f, "incorrect accidental"),
            Error::OctaveNotInRange => write!(f, "octave not in range"),
        }
    }
}

/// Struct representing musical note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    letter: Letter,
    octave: Octave,
    accidental: Accidental,
}

impl Note {
    /// Create new note.
    pub fn new(letter: Letter, octave: Octave, accidental: Accidental) -> Result<Self, Error> {
        match accidental {
            Accidental::None => Ok(Note {
                letter,
                octave,
                accidental,
            }),
            Accidental::Flat => match letter {
                Letter::C | Letter::F => Err(Error::IncorrectAccidental),
                _ => Ok(Note {
                    letter,
                    octave,
                    accidental,
                }),
            },
            Accidental::Sharp => match letter {
                Letter::E | Letter::B => Err(Error::IncorrectAccidental),
                _ => Ok(Note {
                    letter,
                    octave,
                    accidental,
                }),
            },
        }
    }

    /// Get note letter.
    pub fn letter(&self) -> Letter {
        self.letter
    }

    /// Get note octave.
    pub fn octave(&self) -> Octave {
        self.octave
    }

    /// Get note accidental (sharp - ♯ or flat - ♭).
    pub fn accidental(&self) -> Accidental {
        self.accidental
    }

    /// Get note with the same pitch but different accidental (or exactly the same note if there isn't one).
    ///
    /// For example: for C♯ - D♭ is returned.
    pub fn enharmonic(self) -> Note {
        match self.accidental {
            Accidental::None => self,
            Accidental::Flat => Note {
                letter: self.letter.previous(),
                octave: self.octave,
                accidental: Accidental::Sharp,
            },
            Accidental::Sharp => Note {
                letter: self.letter.next(),
                octave: self.octave,
                accidental: Accidental::Flat,
            },
        }
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = Pitch::from(*self);
        let b = Pitch::from(*other);
        a.partial_cmp(&b)
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = Pitch::from(*self);
        let b = Pitch::from(*other);
        a.cmp(&b)
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.letter, self.accidental, self.octave)
    }
}

impl From<Pitch> for Note {
    fn from(pitch: Pitch) -> Self {
        let number = pitch.number();
        let octave = pitch.octave();
        match number {
            0 => Note::new(Letter::C, octave, Accidental::None),
            1 => Note::new(Letter::C, octave, Accidental::Sharp),
            2 => Note::new(Letter::D, octave, Accidental::None),
            3 => Note::new(Letter::D, octave, Accidental::Sharp),
            4 => Note::new(Letter::E, octave, Accidental::None),
            5 => Note::new(Letter::F, octave, Accidental::None),
            6 => Note::new(Letter::F, octave, Accidental::Sharp),
            7 => Note::new(Letter::G, octave, Accidental::None),
            8 => Note::new(Letter::G, octave, Accidental::Sharp),
            9 => Note::new(Letter::A, octave, Accidental::None),
            10 => Note::new(Letter::A, octave, Accidental::Sharp),
            11 => Note::new(Letter::B, octave, Accidental::None),
            _ => unreachable!(),
        }
        .unwrap()
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        let letter = note.letter();
        let index = match letter {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 7,
            Letter::A => 9,
            Letter::B => 11,
        };

        let octave: u8 = note.octave().into();
        let mut index = octave * 12 + index;

        let accidental = note.accidental();
        match accidental {
            Accidental::Flat => index -= 1,
            Accidental::Sharp => index += 1,
            _ => (),
        }

        PITCHES[index as usize]
    }
}

/// Musical note letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Letter {
    /// Get previous note letter.
    pub fn previous(&self) -> Letter {
        match self {
            Letter::C => Letter::B,
            Letter::D => Letter::C,
            Letter::E => Letter::D,
            Letter::F => Letter::A,
            Letter::G => Letter::F,
            Letter::A => Letter::G,
            Letter::B => Letter::A,
        }
    }

    /// Get next note letter.
    pub fn next(&self) -> Letter {
        match self {
            Letter::C => Letter::D,
            Letter::D => Letter::E,
            Letter::E => Letter::F,
            Letter::F => Letter::G,
            Letter::G => Letter::A,
            Letter::A => Letter::B,
            Letter::B => Letter::C,
        }
    }
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Letter::C => 'C',
            Letter::D => 'D',
            Letter::E => 'E',
            Letter::F => 'F',
            Letter::G => 'G',
            Letter::A => 'A',
            Letter::B => 'B',
        };
        write!(f, "{}", character)
    }
}

impl TryFrom<char> for Letter {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'c' | 'C' => Ok(Letter::C),
            'd' | 'D' => Ok(Letter::D),
            'e' | 'E' => Ok(Letter::E),
            'f' | 'F' => Ok(Letter::F),
            'g' | 'G' => Ok(Letter::G),
            'a' | 'A' => Ok(Letter::A),
            'b' | 'B' => Ok(Letter::B),
            _ => Err(Error::IncorrectLetter),
        }
    }
}

/// Supported octaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Octave {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

impl Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Octave::First => '₀',
            Octave::Second => '₁',
            Octave::Third => '₂',
            Octave::Fourth => '₃',
            Octave::Fifth => '₄',
            Octave::Sixth => '₅',
            Octave::Seventh => '₆',
            Octave::Eighth => '₇',
            Octave::Ninth => '₈',
            Octave::Tenth => '₉',
        };
        write!(f, "{}", character)
    }
}

impl TryFrom<u8> for Octave {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Octave::First),
            1 => Ok(Octave::Second),
            2 => Ok(Octave::Third),
            3 => Ok(Octave::Fourth),
            4 => Ok(Octave::Fifth),
            5 => Ok(Octave::Sixth),
            6 => Ok(Octave::Seventh),
            7 => Ok(Octave::Eighth),
            8 => Ok(Octave::Ninth),
            9 => Ok(Octave::Tenth),
            _ => Err(Error::OctaveNotInRange),
        }
    }
}

impl From<Octave> for u8 {
    fn from(octave: Octave) -> Self {
        match octave {
            Octave::First => 0,
            Octave::Second => 1,
            Octave::Third => 2,
            Octave::Fourth => 3,
            Octave::Fifth => 4,
            Octave::Sixth => 5,
            Octave::Seventh => 6,
            Octave::Eighth => 7,
            Octave::Ninth => 8,
            Octave::Tenth => 9,
        }
    }
}

/// Musical note accidental.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    /// No accidental.
    None,
    /// Flat - ♭.
    Flat,
    /// Sharp - ♯.
    Sharp,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accidental::None => write!(f, ""),
            Accidental::Flat => write!(f, "♭"),
            Accidental::Sharp => write!(f, "♯"),
        }
    }
}
