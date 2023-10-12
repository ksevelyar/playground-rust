pub fn find_next_square(sq: u64) -> Option<u64> {
    let sqrt = (sq as f64).sqrt();
    let is_perfect_square = sqrt.fract() == 0.0;

    is_perfect_square.then(|| (sqrt + 1.0).powi(2) as u64)
}
