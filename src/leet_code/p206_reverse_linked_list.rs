use crate::data_struct::list_node::ListNode;

struct Solution;

impl Solution {
    /// [206. 反转链表](https://leetcode.cn/problems/reverse-linked-list/)
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(
            head: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if let Some(mut head) = head {
                let tail = head.next.take();
                head.next = prev;

                reverse(tail, Some(head))
            } else {
                prev
            }
        }

        reverse(head, None)
        // 0ms/2.3MB
    }

    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let (mut prev, mut next) = (None, None);
    //     let mut tail = head;

    //     while let Some(head) = tail.take() {
    //         next = Some(Box::new(ListNode {
    //             val: head.val,
    //             next: prev.take(),
    //         }));
    //         prev = next;

    //         tail = head.next;
    //     }

    //     prev
    //     // 0ms/2.4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_list(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])))),
            Some(Box::new(ListNode::from(vec![5, 4, 3, 2, 1])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_list(Some(Box::new(ListNode::from(vec![1, 2])))),
            Some(Box::new(ListNode::from(vec![2, 1])))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
