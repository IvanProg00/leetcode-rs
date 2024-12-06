//! [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring)

pub fn longest_palindrome(s: String) -> String {
    let sb = s.as_bytes();
    let mut pos = (0, 0);

    for i in 0..sb.len() * 2 {
        let (mut l, mut r) = (i / 2, i / 2 + (i % 2 != 0) as usize);
        while l <= r && l < sb.len() && r < sb.len() {
            if sb[l] == sb[r] {
                if r - l > pos.1 - pos.0 {
                    pos = (l, r)
                }
            } else {
                break;
            }

            if l == 0 {
                break;
            }

            l -= 1;
            r += 1;
        }
    }

    s[pos.0..=pos.1].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case("babad", "bab")]
    #[case("cbbd", "bb")]
    fn test_longest_palindrome(#[case] s: String, #[case] expected: String) {
        let result = longest_palindrome(s);
        assert_eq!(result, expected);
    }
}
