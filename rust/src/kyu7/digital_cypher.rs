pub fn encode(msg: String, key: i32) -> Vec<i32> {
    let msg_digits = msg.chars().map(|ch| ch as i32 - 'a' as i32 + 1);

    msg_digits
        .zip(
            key.to_string()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as i32)
                .cycle(),
        )
        .map(|(msg_digit, key_digit)| msg_digit + key_digit)
        .collect()
}
