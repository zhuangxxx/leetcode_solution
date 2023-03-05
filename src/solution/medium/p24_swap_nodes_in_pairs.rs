use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [24. 两两交换链表中的节点](https://leetcode.cn/problems/swap-nodes-in-pairs/)
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut swap = ListNode::new(0);
        let mut tail = &mut swap;
        while let Some(node) = head.take() {
            if let Some(mut next) = node.next {
                head = next.next.take();
                next.next = Some(Box::new(ListNode::new(node.val)));
                tail.next = Some(next);
                tail = tail.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                tail.next = Some(node);
                break;
            }
        }

        swap.next
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::swap_pairs(Some(Box::new(ListNode::from(vec![1, 2, 3, 4])))),
            Some(Box::new(ListNode::from(vec![2, 1, 4, 3])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::swap_pairs(None), None);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::swap_pairs(Some(Box::new(ListNode::new(1)))),
            Some(Box::new(ListNode::new(1)))
        );
    }
}
