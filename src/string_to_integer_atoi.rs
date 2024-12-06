//! [String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi)

pub fn my_atoi(s: String) -> i32 {
    let s = s.trim_start();

    let (s, sign) = match s.strip_prefix('-') {
        Some(s) => (s, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1),
    };

    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc, d| {
            acc.saturating_mul(10).saturating_add(sign * d as i32)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("42", 42)]
    #[case("-042", -42)]
    #[case("1337c0d3", 1337)]
    #[case("0-1", 0)]
    #[case("words and 987", 0)]
    fn test_my_atoi(#[case] s: String, #[case] expected: i32) {
        let result = my_atoi(s);
        assert_eq!(result, expected);
    }
}
