//! # Gen Random
//!
//! `gen_random` is a lib for generating random number by custom its length and kind

use rand::Rng;

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
        Custom {
            length,
            kind,
        }
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
            CharSetKind::Number => {
                b"0123456789".to_vec()
            }
            CharSetKind::Letter => {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            ".to_vec()
            }
            CharSetKind::Symbol => {
                b"~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec()
            }
            CharSetKind::NumberAndLetter => {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789".to_vec()
            }
            CharSetKind::NumberAndSymbol => {
                b"0123456789~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec()
            }
            CharSetKind::LetterAndSymbol => {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                           ~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec()
            }
            CharSetKind::NumberLetterAndSymbol => {
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789~!@#$%^&*()_+-={}[]|:;<>,.?/\"\\".to_vec()
            }
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

