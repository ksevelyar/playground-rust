pub fn xo(string: &'static str) -> bool {
    let balance = string.chars().fold(0, |balance, ch| match ch {
        'x' | 'X' => balance + 1,
        'o' | 'O' => balance - 1,
        _ => balance,
    });

    balance == 0
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
