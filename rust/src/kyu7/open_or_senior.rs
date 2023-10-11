const MIN_SENIOR_AGE: i32 = 55;
const MIN_SENIOR_HANDICAP: i32 = 7;

pub fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.iter()
        .map(|member| match member {
            (age, handicap) if is_senior(*age, *handicap) => "Senior".to_string(),
            _ => "Open".to_string(),
        })
        .collect()
}

fn is_senior(age: i32, handicap: i32) -> bool {
    age >= MIN_SENIOR_AGE && handicap > MIN_SENIOR_HANDICAP
}
