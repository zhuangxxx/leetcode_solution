use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [61. 旋转链表](https://leetcode.cn/problems/rotate-list/)
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        let mut tail = &head;

        let mut len = 0;
        while let Some(node) = tail {
            len += 1;
            tail = &node.next;
        }

        if len <= 1 || k % len == 0 {
            return head;
        }

        if k >= len {
            k %= len;
        }

        let mut rotate = ListNode::new(0);

        let mut tail = &mut head;
        while let Some(node) = tail {
            len -= 1;
            if len == k {
                rotate.next = node.next.take();
                break;
            }

            tail = &mut node.next;
        }

        let mut tail = &mut rotate.next;
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = head;
                break;
            }

            tail = &mut node.next;
        }

        rotate.next
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::rotate_right(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5]))), 2),
            Some(Box::new(ListNode::from(vec![4, 5, 1, 2, 3])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::rotate_right(Some(Box::new(ListNode::from(vec![0, 1, 2]))), 4),
            Some(Box::new(ListNode::from(vec![2, 0, 1])))
        );
    }
}
