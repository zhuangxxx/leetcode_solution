/// [232. 用栈实现队列](https://leetcode.cn/problems/implement-queue-using-stacks/)
struct MyQueue {
    ord: std::collections::VecDeque<i32>,
    rev: std::collections::VecDeque<i32>,
    // 0ms/2.2MB
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            ord: std::collections::VecDeque::new(),
            rev: std::collections::VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.ord.push_back(x);
        self.rev = std::collections::VecDeque::new();

        for i in (0..self.ord.len()).rev() {
            self.rev.push_back(self.ord[i]);
        }
    }

    fn pop(&mut self) -> i32 {
        self.ord = std::collections::VecDeque::new();

        for i in (0..self.rev.len() - 1).rev() {
            self.ord.push_back(self.rev[i])
        }

        self.rev.pop_back().unwrap()
    }

    fn peek(&self) -> i32 {
        self.ord[0]
    }

    fn empty(&self) -> bool {
        self.ord.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_queue = MyQueue::new();
        my_queue.push(1);
        my_queue.push(2);
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.pop(), 1);
        assert!(!my_queue.empty());
    }
}
