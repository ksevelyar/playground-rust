pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    a1.iter()
        .flat_map(|word_a1| a2.iter().map(|word_a2| words_length_diff(word_a1, word_a2)))
        .max()
        .unwrap_or(-1)
}

fn words_length_diff(word1: &str, word2: &str) -> i32 {
    (word1.len() as i32 - word2.len() as i32).abs()
}
