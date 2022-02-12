pub const WORD_LENGTH: usize = 5;
pub fn check_alphabet(c: &char) -> bool {
    char::is_ascii_lowercase(c)
}
