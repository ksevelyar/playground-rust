pub fn balanced_num(n: u64) -> String {
    if n <= 19 { return "Balanced".to_string() }

    let n = n.to_string();
    let middle = n.len() / 2;
    let (left, right) = match middle % 2 == 0 {
        true => dbg!((&n[..middle - 1], &n[middle + 1..])),
        false => (&n[..middle], &n[middle + 1..]),
    };

    match sum(left) == sum(right) {
        true => "Balanced",
        false => "Not Balanced",
    }
    .to_string()
}

fn sum(digits: &str) -> u64 {
    digits.chars().map(|ch| ch.to_digit(10).unwrap() as u64).sum()
}
