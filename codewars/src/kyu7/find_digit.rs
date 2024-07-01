pub fn find_digit(num: i32, nth: i32) -> i32 {
    if nth < 1 {
        return -1;
    }

    num.abs()
        .to_string()
        .chars()
        .rev()
        .enumerate()
        .find(|(ind, _)| (*ind as i32) + 1 == nth)
        .map(|(_, digit)| digit.to_digit(10).unwrap() as i32)
        .unwrap_or(0)
}
