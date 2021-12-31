use std::fmt;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Overlap {
    pub right_place: u8,
    pub wrong_place: u8,
}

impl Overlap {
    pub fn new(wrong_place: u8, right_place: u8) -> Overlap {
        Overlap {
            wrong_place,
            right_place,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Clue {
    pub value: usize,
    pub word: Vec<char>,
}

impl fmt::Debug for Clue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clue")
            .field("value", &self.value)
            .field("word", &self.word.iter().collect::<String>())
            .finish()
    }
}