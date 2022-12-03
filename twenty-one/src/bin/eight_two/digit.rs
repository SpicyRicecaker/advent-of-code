use std::collections::HashSet;

/// A representation of a digit, which includes its value and constituent
/// characters
#[derive(Debug, Clone)]
pub struct Digit {
    /// values
    pub characters: HashSet<char>,
}

impl Digit {
    pub fn get_first_char(&self) -> char {
        *self.characters.iter().next().unwrap()
    }
}

impl From<String> for Digit {
    fn from(s: String) -> Self {
        Self {
            characters: s.chars().into_iter().collect(),
        }
    }
}

/// see https://doc.rust-lang.org/rust-by-example/trait/ops.html
impl std::ops::Add<Digit> for Digit {
    type Output = Digit;

    // combine the characters in the digit, with no repeats
    fn add(self, rhs: Digit) -> Self::Output {
        let mut characters = self.characters;
        rhs.characters.into_iter().for_each(|c| {
            characters.insert(c);
        });

        Self {
            characters
        }
    }
}

impl std::ops::Sub<Digit> for Digit {
    type Output = Digit;

    // combine the characters in the digit, with no repeats
    fn sub(self, rhs: Digit) -> Self::Output {
        let mut characters = self.characters;
        rhs.characters.into_iter().for_each(|c| {
            characters.remove(&c);
        });

        Self {
            characters
        }
    }
}

impl std::ops::Add<char> for Digit {
    type Output = Digit;

    // combine the characters in the digit, with no repeats
    fn add(self, rhs: char) -> Self::Output {
        let mut characters = self.characters;
        characters.insert(rhs);

        Self {
            characters
        }
    }
}

impl std::ops::Sub<char> for Digit {
    type Output = Digit;

    // combine the characters in the digit, with no repeats
    fn sub(self, rhs: char) -> Self::Output {
        let mut characters = self.characters;
        characters.remove(&rhs);

        Self {
            characters
        }
    }
}