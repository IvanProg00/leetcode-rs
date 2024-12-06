//! [Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree)

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Gets an integer array nums where the elements are sorted in ascending order,
/// and converts it to a height-balanced binary search tree.
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mid = nums.len() / 2;

    let mut root = TreeNode::new(nums.get(mid).cloned()?);
    root.left = sorted_array_to_bst(nums[..mid].to_vec());
    root.right = sorted_array_to_bst(nums[mid + 1..].to_vec());

    Some(Rc::new(RefCell::new(root)))
}
