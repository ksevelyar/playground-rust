pub fn disarium_number(n: u32) -> String {
    let sum: u32 = n
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .enumerate()
        .map(|(ind, digit)| digit.pow((ind + 1) as u32))
        .sum();

    if sum == n {
        "Disarium !!".to_string()
    } else {
        "Not !!".to_string()
    }
}
