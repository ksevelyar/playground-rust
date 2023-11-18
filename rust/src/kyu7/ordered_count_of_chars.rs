pub fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    sip.chars().fold(vec![], |mut acc, ch| {
        let count = sip.chars().filter(|inner_ch| *inner_ch == ch).count() as i32;
        let tuple = (ch, count);

        if acc.contains(&tuple) {
            acc
        } else {
            acc.push(tuple);
            acc
        }
    })
}
