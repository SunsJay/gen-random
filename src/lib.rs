/// Generate random number by custom

use rand::Rng;

pub struct Custom {
    /// length of random number or string
    length: usize,
    kind: CharSetKind,
}


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


//#[cfg(test)]
//mod tests {
//    use crate::{CharSetKind, Custom};
//
//
//    #[test]
//    fn number() {
//        let random = Custom::new(5, CharSetKind::Number).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn letter() {
//        let random = Custom::new(5, CharSetKind::Letter).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn symbol() {
//        let random = Custom::new(5, CharSetKind::Symbol).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn number_and_letter() {
//        let random = Custom::new(5, CharSetKind::NumberAndLetter).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn number_and_symbol() {
//        let random = Custom::new(5, CharSetKind::NumberAndSymbol).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn letter_and_symbol() {
//        let random = Custom::new(5, CharSetKind::LetterAndSymbol).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//    #[test]
//    fn number_and_letter_and_symbol() {
//        let random = Custom::new(10, CharSetKind::NumberLetterAndSymbol).generate();
//        assert_eq!(random, "a".to_string())
//    }
//
//
//}
