//! [Contains Duplicate](https://leetcode.com/problems/contains-duplicate)

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::with_capacity(nums.len());

    for n in nums {
        if set.contains(&n) {
            return true;
        }

        set.insert(n);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(vec![1, 2, 3, 1], true)]
    #[case(vec![1, 2, 3, 4], false)]
    #[case(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true)]
    fn test_contains_duplicate(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let result = contains_duplicate(nums);
        assert_eq!(result, expected);
    }
}
