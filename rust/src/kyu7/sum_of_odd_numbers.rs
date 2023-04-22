fn row_sum_odd_numbers(n: i64) -> i64 {
    let start_position: usize = (1..(n as usize)).sum();

    (1..)
        .filter(|num| num % 2 != 0)
        .skip(start_position)
        .take(n as usize)
        .sum()
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(3), 27);
    assert_eq!(row_sum_odd_numbers(4), 64);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
