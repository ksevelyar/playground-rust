pub fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut words: Vec<_> = s.split_whitespace().map(|word| word.to_string()).collect();

    words.sort_by_key(|word| word.chars().last());

    words
}
