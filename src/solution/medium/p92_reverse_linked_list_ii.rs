use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [92. 反转链表 II](https://leetcode.cn/problems/reverse-linked-list-ii/)
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }

        let mut rev = ListNode::new(0);
        let mut ptr = &mut rev;
        let mut p = 1;
        let mut last: *mut _ = unsafe { std::mem::zeroed() };
        while let Some(mut node) = head {
            head = node.next.take();

            if p >= left && p <= right {
                node.next = ptr.next.take();
            }
            ptr.next = Some(node);

            if p == left {
                last = ptr.next.as_mut().unwrap().as_mut();
            } else if p == right {
                ptr = unsafe { &mut *last };
            } else if p < left || p > right {
                ptr = ptr.next.as_mut().unwrap();
            }

            p += 1;
        }

        rev.next
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_between(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5]))), 2, 4),
            Some(Box::new(ListNode::from(vec![1, 4, 3, 2, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_between(Some(Box::new(ListNode::new(5))), 1, 1),
            Some(Box::new(ListNode::new(5)))
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::reverse_between(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5]))), 2, 2),
            Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])))
        );
    }
}
