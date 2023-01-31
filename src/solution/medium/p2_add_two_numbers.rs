use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [2. 两数相加](https://leetcode.cn/problems/add-two-numbers/)
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        let mut one = false;
        loop {
            let s = match (l1.take(), l2.take()) {
                (Some(t1), Some(t2)) => {
                    l1 = t1.next;
                    l2 = t2.next;
                    t1.val + t2.val
                }
                (Some(t), None) | (None, Some(t)) => {
                    l1 = t.next;
                    t.val
                }
                _ => break,
            } + if one { 1 } else { 0 };

            one = s >= 10;
            tail.next = Some(Box::new(ListNode::new(s % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        if one {
            tail.next = Some(Box::new(ListNode::new(1)));
        }

        head.next
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![2, 4, 3]))),
                Some(Box::new(ListNode::from(vec![5, 6, 4])))
            ),
            Some(Box::new(ListNode::from(vec![7, 0, 8])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![0]))),
                Some(Box::new(ListNode::from(vec![0])))
            ),
            Some(Box::new(ListNode::from(vec![0])))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode::from(vec![9, 9, 9, 9, 9, 9, 9]))),
                Some(Box::new(ListNode::from(vec![9, 9, 9, 9])))
            ),
            Some(Box::new(ListNode::from(vec![8, 9, 9, 9, 0, 0, 0, 1])))
        );
    }
}
