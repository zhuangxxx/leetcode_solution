/// [225. 用队列实现栈](https://leetcode.cn/problems/implement-stack-using-queues/)
struct MyStack {
    ord: std::collections::VecDeque<i32>,
    rev: std::collections::VecDeque<i32>,
    // 0ms/2.2MB
}

impl MyStack {
    fn new() -> Self {
        MyStack {
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

        for i in (1..self.rev.len()).rev() {
            self.ord.push_back(self.rev[i])
        }

        self.rev.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        self.rev[0]
    }

    fn empty(&self) -> bool {
        self.rev.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_stack = MyStack::new();
        my_stack.push(1);
        my_stack.push(2);
        assert_eq!(my_stack.top(), 2);
        assert_eq!(my_stack.pop(), 2);
        assert!(!my_stack.empty());
    }
}
