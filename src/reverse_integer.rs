//! [Reverse Integer](https://leetcode.com/problems/reverse-integer)

pub fn reverse(x: i32) -> i32 {
    let mut temp = x.abs();
    let mut num = 0i32;

    while temp > 0 {
        let digit = temp % 10;
        if let Some(new_num) = num.checked_mul(10).and_then(|n| n.checked_add(digit)) {
            num = new_num;
        } else {
            return 0;
        }

        temp /= 10;
    }

    if x.is_positive() {
        num
    } else {
        -num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(123, 321)]
    #[case(-123, -321)]
    #[case(120, 21)]
    #[case(1534236469, 0)]
    fn test_is_palindrome(#[case] input: i32, #[case] expected: i32) {
        let result = reverse(input);
        assert_eq!(result, expected);
    }
}
