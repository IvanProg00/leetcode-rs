//! [3Sum](https://leetcode.com/problems/3sum)

use std::cmp::Ordering;

/// Gets an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
/// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut result = Vec::new();
    let n = nums.len();

    for i in 0..(n - 2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let (mut l, mut r) = (i + 1, n - 1);

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            match sum.cmp(&0) {
                Ordering::Equal => {
                    result.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;

                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                }
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2], vec![-1,0,1]])]
    #[case(vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6], vec![vec![-4,-2,6],vec![-4,0,4],vec![-4,1,3],vec![-4,2,2],vec![-2,-2,4],vec![-2,0,2]])]
    fn test_title_to_number(#[case] nums: Vec<i32>, #[case] mut expected: Vec<Vec<i32>>) {
        let mut result = three_sum(nums);

        for n in result.iter_mut() {
            n.sort_unstable();
        }

        for n in expected.iter_mut() {
            n.sort_unstable();
        }

        assert_eq!(result, expected);
    }
}
