const CORRECT_ANSWER_SCORE: i64 = 4;
const INCORRECT_ANSWER_SCORE: i64 = -1;
const BLANK_ANSWER_SCORE: i64 = 0;

pub fn check_exam(checks: &[&str], answers: &[&str]) -> i64 {
    answers
        .iter()
        .zip(checks)
        .map(|answer_with_check| match answer_with_check {
            (answer, check) if answer == check => CORRECT_ANSWER_SCORE,
            (&"", _) => BLANK_ANSWER_SCORE,
            _ => INCORRECT_ANSWER_SCORE,
        })
        .sum::<i64>()
        .max(0)
}
