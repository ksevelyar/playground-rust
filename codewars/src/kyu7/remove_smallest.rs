pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.is_empty() {
        return vec![];
    }

    let min = numbers.iter().min().unwrap();
    let first_index = numbers.iter().position(|num| num == min).unwrap();

    numbers
        .iter()
        .enumerate()
        .filter(|(ind, _)| *ind != first_index)
        .map(|(_, value)| *value)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::remove_smallest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u32], expected: &[u32]) {
        assert_eq!(
            remove_smallest(a),
            expected,
            "{ERR_MSG} with numbers = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5], &[2, 3, 4, 5]);
        dotest(&[1, 2, 3, 4], &[2, 3, 4]);
        dotest(&[5, 3, 2, 1, 4], &[5, 3, 2, 4]);
        dotest(&[1, 2, 3, 1, 1], &[2, 3, 1, 1]);
    }
}
