fn collatz(n: u64) -> u64 {
    match n {
        1 => 1,
        x if x % 2 == 0 => 1 + collatz(n / 2),
        _ => 1 + collatz(n * 3 + 1),
    }
}
