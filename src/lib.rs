//! # Gen Random
//!
//! `gen_random` is a lib for generating random number by custom its length and kind.
//! It's very easy to generate different randon number, Number/Letter/Symbol or collection of them.

use rand::Rng;

const NUMBER: &'static [u8] = b"0123456789";
const SYMBOL: &'static [u8] = b"~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\";
const LETTER: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Custom length and kind of random number
pub struct Custom {
    /// Length of random number
    length: usize,
    /// Kind of random number
    kind: CharSetKind,
}

/// Kinds of random number
pub enum CharSetKind {
    Number,
    Letter,
    Symbol,
    NumberAndLetter,
    NumberAndSymbol,
    LetterAndSymbol,
    NumberLetterAndSymbol,
}

impl Custom {
    pub fn new(length: usize, kind: CharSetKind) -> Custom {
        Custom { length, kind }
    }

    /// Generate random number by custom its length and kind
    ///
    /// # Examples
    ///
    /// ```
    ///  let random = gen_random::Custom::new(5, gen_random::CharSetKind::Number).generate();
    ///  assert_eq!(random, "12345".to_string())
    /// ```
    pub fn generate(&self) -> String {
        let charset = match self.kind {
            CharSetKind::Number => NUMBER.to_vec(),
            CharSetKind::Letter => LETTER.to_vec(),
            CharSetKind::Symbol => SYMBOL.to_vec(),
            CharSetKind::NumberAndLetter => [NUMBER, LETTER].concat().to_vec(),
            CharSetKind::NumberAndSymbol => [NUMBER, SYMBOL].concat().to_vec(),
            CharSetKind::LetterAndSymbol => [LETTER, SYMBOL].concat().to_vec(),
            CharSetKind::NumberLetterAndSymbol => [NUMBER, LETTER, SYMBOL].concat().to_vec(),
        };

        let mut rng = rand::thread_rng();

        let value: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();

        value
    }
}

