pub fn get_middle(s: &str) -> &str {
    let mid = s.len() / 2;
    let range = match s.len() % 2 == 0 {
        true => (mid - 1)..=mid,
        false => mid..=mid,
    };

    &s[range]
}
