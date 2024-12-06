//! [Open the Lock](https://leetcode.com/problems/open-the-lock)

use std::collections::VecDeque;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited = [false; 10_000];

    deadends
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .for_each(|n| visited[n] = true);
    if visited[0] {
        return -1;
    }

    let target = target.parse::<usize>().unwrap_or_default();
    visited[0] = true;
    let mut queue = VecDeque::from([(0, 0)]);

    while let Some((num, count)) = queue.pop_front() {
        if num == target {
            return count;
        }

        queue.extend(
            [1, 10, 100, 1000]
                .into_iter()
                .flat_map(|p| {
                    let d = (num / p) % 10;
                    let n = num - (d * p);
                    [1, 9].into_iter().map(move |i| ((d + i) % 10) * p + n)
                })
                .filter(|&n| {
                    if visited[n] {
                        return false;
                    }
                    visited[n] = true;
                    true
                })
                .map(|n| (n, count + 1)),
        )
    }

    -1
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        vec![
            String::from("0201"), String::from("0101"),
            String::from("0102"), String::from("1212"), String::from("2002"),
        ],
        "0202",
        6,
    )]
    #[case(vec![String::from("8888")], "0009", 1)]
    #[case(
        vec![
            String::from("8887"), String::from("8889"),
            String::from("8878"), String::from("8898"),
            String::from("8788"), String::from("8988"),
            String::from("7888"), String::from("9888")
        ],
         "8888",
        -1,
    )]
    fn test_open_look(
        #[case] deadends: Vec<String>,
        #[case] target: String,
        #[case] expected: i32,
    ) {
        let result = open_lock(deadends, target);
        assert_eq!(result, expected);
    }
}
