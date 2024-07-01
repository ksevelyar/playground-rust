pub fn add(num1: u32, num2: u32) -> u64 {
    let num1 = digits(num1);
    let num2 = digits(num2);

    let max_len = match num1.len() > num2.len() {
        true => num1.len(),
        false => num2.len(),
    };

    (0..max_len)
        .map(|ind| (num1.get(ind).unwrap_or(&0) + num2.get(ind).unwrap_or(&0)).to_string())
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .rev()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}
