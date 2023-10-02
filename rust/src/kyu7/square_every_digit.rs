pub fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|ch| (ch.to_digit(10).unwrap().pow(2)).to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}
