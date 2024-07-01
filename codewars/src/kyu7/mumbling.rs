pub fn accum(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .map(repeat_and_camelize)
        .collect::<Vec<String>>()
        .join("-")
}

fn repeat_and_camelize((ind, ch): (usize, char)) -> String {
    let camelized_ch = ch.to_ascii_uppercase();
    let other_chars = ch.to_ascii_lowercase().to_string().repeat(ind);

    format!("{camelized_ch}{other_chars}")
}

#[test]
fn basic_tests() {
    assert_eq!(accum("cwAt"), "C-Ww-Aaa-Tttt");
}
fn calc(s: &str) -> u32 {
    let sevens: u32 = s
        .as_bytes()
        .iter()
        .map(|byte| byte.to_string().chars().filter(|ch| *ch == '7').count() as u32)
        .sum();

    sevens * 6
}
