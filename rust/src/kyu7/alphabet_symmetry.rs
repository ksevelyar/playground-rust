pub fn solve(strings: &[String]) -> Vec<usize> {
    strings
        .iter()
        .map(|string| {
            string
                .to_lowercase()
                .chars()
                .zip('a'..='z')
                .filter(|(a, b)| a == b)
                .count()
        })
        .collect()
}
