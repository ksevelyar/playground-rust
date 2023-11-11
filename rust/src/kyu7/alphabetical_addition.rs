pub fn add_letters(letters: Vec<char>) -> char {
    if letters.is_empty() {
        return 'z';
    }

    let shift = letters
        .iter()
        .map(|ch| *ch as u32 - ('a' as u32 - 1))
        .sum::<u32>()
        - 1;

    (b'a' + (shift as u8 % 26)) as char
}
