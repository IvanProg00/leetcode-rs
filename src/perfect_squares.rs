//! [Perfect Squares](https://leetcode.com/problems/perfect-squares)

pub fn num_squares(n: i32) -> i32 {
    let n = n as usize;
    let max = (n as f32).sqrt().floor() as usize;

    let mut dp = vec![i32::MAX / 2; n + 1];

    for x in 1..=max {
        dp[x * x] = 1;
    }

    for x in 1..=n {
        for i in 1..x {
            dp[x] = dp[x].min(dp[i] + dp[x - i]);
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(12, 3)]
    #[case(13, 2)]
    fn test_num_squares(#[case] n: i32, #[case] expected: i32) {
        let result = num_squares(n);
        assert_eq!(result, expected);
    }
}
