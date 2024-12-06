//! [Divide Two Integers](https://leetcode.com/problems/divide-two-integers)

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let is_neg = (dividend < 0) ^ (divisor < 0);

    let mut dividend = if dividend > 0 { -dividend } else { dividend };
    let divisor = if divisor > 0 { -divisor } else { divisor };

    let mut result = 0;

    for shift in (0..divisor.leading_ones()).rev() {
        if dividend <= divisor << shift {
            dividend -= divisor << shift;
            result += -1 << shift;
        }
    }

    if is_neg {
        result
    } else if result == i32::MIN {
        i32::MAX
    } else {
        -result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(10, 3, 3)]
    #[case(7, -3, -2)]
    #[case(23, 3, 7)]
    fn test_divide(#[case] dividend: i32, #[case] divisor: i32, #[case] expected: i32) {
        let result = divide(dividend, divisor);
        assert_eq!(result, expected);
    }
}
