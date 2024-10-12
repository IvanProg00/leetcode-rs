//! [Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)

/// Generates all combinations of well-formed parentheses of n pairs of parentheses.
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    generate_parenthesis_inner(String::new(), n, n, n)
}

fn generate_parenthesis_inner(s: String, n: i32, openers: i32, closers: i32) -> Vec<String> {
    if openers > n || openers < 0 || closers > n || closers < 0 || openers > closers {
        return Vec::new();
    }

    if openers == 0 && closers == 0 {
        return vec![s];
    }

    let mut s1 = s.clone();
    s1.push('(');
    let mut parenthesis_1 = generate_parenthesis_inner(s1, n, openers - 1, closers);

    let mut s2 = s.clone();
    s2.push(')');
    let parenthesis_2 = generate_parenthesis_inner(s2, n, openers, closers - 1);

    parenthesis_1.extend(parenthesis_2);
    parenthesis_1
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, vec![
        String::from("((()))"),
        String::from("(()())"),
        String::from("(())()"),
        String::from("()(())"),
        String::from("()()()"),
    ])]
    #[case(1, vec![String::from("()")])]
    fn test_generate_parenthesis(#[case] n: i32, #[case] expected: Vec<String>) {
        let result = generate_parenthesis(n);
        assert_eq!(result, expected);
    }
}
