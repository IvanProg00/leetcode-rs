//! [Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list)

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head?;
    let mut cur = head.as_mut();

    while let Some(next) = cur.next.as_mut() {
        if cur.val == next.val {
            cur.next = next.next.take();
        } else if let Some(next) = cur.next.as_mut() {
            cur = next;
        } else {
            break;
        }
    }

    Some(head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(vec![1, 2, 3], vec![1, 2, 3])]
    #[case(vec![1, 1, 2], vec![1, 2])]
    #[case(vec![1, 1, 2, 3, 3], vec![1, 2, 3])]
    fn test_delete_duplicates(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let mut head = None;

        for n in input.into_iter().rev() {
            head = Some(Box::new(ListNode { val: n, next: head }));
        }

        let mut result = delete_duplicates(head);
        let mut result_vec = Vec::new();

        while let Some(node) = result {
            result_vec.push(node.val);
            result = node.next;
        }

        assert_eq!(result_vec, expected);
    }
}
