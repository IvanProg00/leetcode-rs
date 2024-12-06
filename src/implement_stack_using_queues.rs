//! [Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues)

pub struct MyStack {
    arr: Vec<i32>,
}

impl MyStack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { arr: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.arr.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.arr.pop().unwrap_or(0)
    }

    pub fn top(&self) -> i32 {
        *self.arr.last().unwrap_or(&0i32)
    }

    pub fn empty(&self) -> bool {
        self.arr.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let mut my_stack = MyStack::new();

        my_stack.push(1);
        my_stack.push(2);
        assert_eq!(my_stack.top(), 2);
        assert_eq!(my_stack.pop(), 2);
        assert!(!my_stack.empty());
    }
}
