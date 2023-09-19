const VOWELS: [char; 5] = ['i', 'e', 'o', 'a', 'u'];

pub fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|ch| !VOWELS.contains(&ch.to_ascii_lowercase()))
        .collect()
}
