pub fn power_of_two(x: u64) -> bool {
    if x == 0 {
        return false;
    }

    x & (x - 1) == 0
}
