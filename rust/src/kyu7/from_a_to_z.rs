pub fn gimme_the_letters(sp: &str) -> String {
    let [start, _, end]: [char; 3] = sp.chars().collect::<Vec<char>>().try_into().unwrap();

    (start..=end).collect()
}
