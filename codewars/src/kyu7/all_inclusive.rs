pub fn contain_all_rots(word: &str, arr: Vec<&str>) -> bool {
    if word.is_empty() {
        return true;
    }

    (0..word.len()).all(|ind| {
        let rotated_word = format!("{}{}", &word[ind..], &word[0..ind]);
        arr.contains(&rotated_word.as_str())
    })
}
