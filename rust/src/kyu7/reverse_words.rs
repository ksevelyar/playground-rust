pub fn reverse_words(str: &str) -> String {
    str.split_inclusive(' ').map(reverse_word).collect()
}

fn reverse_word(word: &str) -> String {
    word.chars().fold(String::new(), |mut acc, char| {
        match char.is_whitespace() {
            true => acc.push(char),
            false => acc.insert(0, char),
        }

        acc
    })
}

fn two_sort(arr: &[&str]) -> String {
    let word = arr.iter().min().unwrap();

    word.chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<_>>()
        .join("***")
}

#[test]
fn sample_test() {
    assert_eq!(
        reverse_words("double   spaced   words"),
        "elbuod   decaps   sdrow"
    );
    assert_eq!(
        reverse_words("The quick brown fox jumps over the lazy dog."),
        "ehT kciuq nworb xof spmuj revo eht yzal .god"
    );
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"), "a b c d");
}
