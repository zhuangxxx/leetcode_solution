use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [203. 移除链表元素](https://leetcode.cn/problems/remove-linked-list-elements/)
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut prev = Box::new(ListNode::new(val));
        prev.next = head;
        let mut tail = &mut prev;

        while let Some(next) = tail.next.take() {
            if next.val == val {
                tail.next = next.next;
            } else {
                tail.next = Some(next);
                tail = tail.next.as_mut().unwrap();
            }
        }

        prev.next
        // 4ms/2.8MB
    }

    // pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     let mut head = head;
    //     let mut tail = head.as_mut();

    //     while let Some(node) = tail {
    //         while let Some(next) = node.next.as_mut() {
    //             if next.val != val {
    //                 break;
    //             }

    //             node.next = next.next.take();
    //         }

    //         tail = node.next.as_mut();
    //     }

    //     if let Some(head) = head {
    //         if head.val == val {
    //             head.next
    //         } else {
    //             Some(head)
    //         }
    //     } else {
    //         None
    //     }
    //     // 4ms/2.8MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_elements(Some(Box::new(ListNode::from(vec![1, 2, 6, 3, 4, 5, 6]))), 6),
            Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::remove_elements(None, 1), None);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_elements(Some(Box::new(ListNode::from(vec![7, 7, 7, 7]))), 7),
            None
        );
    }
}
