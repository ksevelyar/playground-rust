pub fn triangle(row_str: &str) -> String {
    let row: Vec<_> = row_str.chars().collect();

    (1..row.len())
        .fold(row, |acc, _step| next_row(&acc))
        .iter()
        .collect()
}

fn new_color(left: char, right: char) -> char {
    if left == right {
        return left;
    }

    *['R', 'G', 'B']
        .iter()
        .find(|ch| **ch != left && **ch != right)
        .unwrap()
}

fn next_row(chars: &[char]) -> Vec<char> {
    chars.windows(2).map(|x| new_color(x[0], x[1])).collect()
}
