use crate::config;

#[derive(Debug, Clone, Copy)]
pub enum DigitHint {
    NotIncluded(char),
    NotHere(usize, char),
    Here(usize, char),
}

impl DigitHint {
    pub fn is_valid(&self) -> bool {
        match self {
            DigitHint::NotIncluded(_) => true,
            DigitHint::NotHere(i, _) => *i < config::WORD_LENGTH,
            DigitHint::Here(i, _) => *i < config::WORD_LENGTH,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TryHint([DigitHint; config::WORD_LENGTH]);

impl TryHint {
    pub fn from_slice(args: &[String]) -> Vec<TryHint> {
        args.chunks_exact(2 * config::WORD_LENGTH)
            .map(|onetry| {
                let mut hint = [DigitHint::NotIncluded('\0'); config::WORD_LENGTH];
                onetry.chunks(2).enumerate().for_each(|(pos, arg)| {
                    // &[char, hint_digit_char]
                    let char = &arg[0];
                    let hint_digit_char = &arg[1];
                    // eprintln!("{:?} {:?} {:?}", pos, char, hint_digit_char);
                    let c = char.to_lowercase().chars().next().unwrap();
                    match hint_digit_char.parse::<usize>().unwrap() {
                        2 => hint[pos] = DigitHint::Here(pos, c),
                        1 => hint[pos] = DigitHint::NotHere(pos, c),
                        0 => hint[pos] = DigitHint::NotIncluded(c),
                        _ => panic!("bad hint digit: {:?}", hint_digit_char),
                    }
                });
                TryHint(hint)
            })
            .collect()
    }
}
