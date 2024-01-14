const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn longest_vowel_chain(s: &str) -> usize {
    s.split(|ch| !VOWELS.contains(&ch))
        .map(|vowels| vowels.len())
        .max()
        .unwrap()
}
