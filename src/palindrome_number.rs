//! [Palindrome Number](https://leetcode.com/problems/palindrome-number)

pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let mut temp_x = x;
    let mut reversed_x = 0;

    while temp_x > 0 {
        reversed_x = reversed_x * 10 + temp_x % 10;
        temp_x /= 10;
    }

    x == reversed_x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(121, true)]
    #[case(-121, false)]
    #[case(10, false)]
    #[case(4839384, true)]
    #[case(123321, true)]
    fn test_is_palindrome(#[case] input: i32, #[case] expected: bool) {
        let result = is_palindrome(input);
        assert_eq!(result, expected);
    }
}
