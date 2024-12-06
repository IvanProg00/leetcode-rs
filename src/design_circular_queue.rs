//! [Design Circular Queue](https://leetcode.com/problems/design-circular-queue)

/// The circular queue is a linear data structure in which the operations are performed
/// based on FIFO (First In First Out) principle, and the last position is connected
/// back to the first position to make a circle. It is also called "Ring Buffer".
pub struct MyCircularQueue {
    pub data: Vec<i32>,
    pub head: i32,
    pub tail: i32,
    pub size: i32,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize],
            head: -1,
            tail: -1,
            size: k,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        } else if self.is_empty() {
            self.head = 0;
        }

        self.tail = (self.tail + 1) % self.size;
        self.data[self.tail as usize] = value;

        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else if self.head == self.tail {
            self.head = -1;
            self.tail = -1;
            true
        } else {
            self.head = (self.head + 1) % self.size;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head as usize]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.tail as usize]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == -1
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.size == self.head
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use Step::*;

    enum Step {
        EnQueue(i32, bool),
        DeQueue(bool),
        Rear(i32),
        Front(i32),
        IsFull(bool),
        IsEmpty(bool),
    }

    #[rstest]
    #[case(3, vec![
        EnQueue(1, true),
        EnQueue(2, true),
        EnQueue(3, true),
        EnQueue(4, false),
        Rear(3),
        IsFull(true),
        DeQueue(true),
        EnQueue(4, true),
        Rear(4)
    ])]
    #[case(4, vec![
        Front(-1),
        EnQueue(1, true),
        EnQueue(2, true),
        DeQueue(true),
        Front(2),
        IsFull(false),
        IsEmpty(false),
        DeQueue(true),
        IsEmpty(true),
        DeQueue(false),
    ])]
    fn test_circular_queue(#[case] init_size: i32, #[case] steps: Vec<Step>) {
        let mut queue = MyCircularQueue::new(init_size);

        for step in &steps {
            match step {
                Step::EnQueue(value, expected) => assert_eq!(queue.en_queue(*value), *expected),
                Step::DeQueue(expected) => assert_eq!(queue.de_queue(), *expected),
                Step::Front(expected) => assert_eq!(queue.front(), *expected),
                Step::Rear(expected) => assert_eq!(queue.rear(), *expected),
                Step::IsFull(expected) => assert_eq!(queue.is_full(), *expected),
                Step::IsEmpty(expected) => assert_eq!(queue.is_empty(), *expected),
            }
        }
    }
}
