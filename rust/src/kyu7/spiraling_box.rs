pub fn create_box(cols: u32, rows: u32) -> Vec<Vec<u32>> {
    (1..=rows)
        .map(|y| {
            let y = match y {
                y if y > rows / 2 => rows - y + 1,
                y => y,
            };
            calc_row(y, cols)
        })
        .collect()
}

fn calc_row(row: u32, cols: u32) -> Vec<u32> {
    (1..=cols)
        .map(|x| match x {
            x if x > row && x <= cols - row => row,
            x if x > cols / 2 => cols - x + 1,
            x => x,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::create_box;

    fn dotest(m: u32, n: u32, expected: &[&[u32]]) {
        let actual = create_box(m, n);
        assert!(
            actual == expected,
            "With m = {m}, n = {n}\nExpected {expected:?}\nBut got {actual:?}"
        )
    }

    #[test]
    fn test_6x4() {
        dotest(
            6,
            4,
            &[
                &[1, 1, 1, 1, 1, 1],
                &[1, 2, 2, 2, 2, 1],
                &[1, 2, 2, 2, 2, 1],
                &[1, 1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test_7x8() {
        dotest(
            7,
            8,
            &[
                &[1, 1, 1, 1, 1, 1, 1],
                &[1, 2, 2, 2, 2, 2, 1],
                &[1, 2, 3, 3, 3, 2, 1],
                &[1, 2, 3, 4, 3, 2, 1],
                &[1, 2, 3, 4, 3, 2, 1],
                &[1, 2, 3, 3, 3, 2, 1],
                &[1, 2, 2, 2, 2, 2, 1],
                &[1, 1, 1, 1, 1, 1, 1],
            ],
        );
    }
}
