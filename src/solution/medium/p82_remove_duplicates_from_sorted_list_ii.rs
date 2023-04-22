use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [82. 删除排序链表中的重复元素 II](https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/)
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut del = ListNode::new(0);
        let mut ptr = &mut del;

        let mut prev = -101;
        while let Some(mut node) = head {
            head = node.next.take();
            if (head.is_some() && head.as_ref().unwrap().val == node.val) || node.val == prev {
                prev = node.val;
            } else {
                prev = node.val;
                ptr.next = Some(node);
                ptr = ptr.next.as_mut().unwrap();
            }
        }

        del.next
        // 0ms/2MB
    }

    // pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut del = ListNode::new(0);
    //     let mut ptr = &mut del;

    //     let (mut tail, mut skip) = (&head, -101);
    //     while let Some(node) = tail {
    //         if node.val == skip {
    //         } else if let Some(next) = &node.next {
    //             if node.val == next.val {
    //                 skip = node.val;
    //             } else {
    //                 ptr.next = Some(Box::new(ListNode::new(node.val)));
    //                 ptr = ptr.next.as_mut().unwrap();
    //             }
    //         } else {
    //             ptr.next = Some(Box::new(ListNode::new(node.val)));
    //         }
    //         tail = &node.next;
    //     }

    //     del.next
    //     // 0ms/1.9MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode::from(vec![1, 2, 3, 3, 4, 4, 5])))),
            Some(Box::new(ListNode::from(vec![1, 2, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode::from(vec![1, 1, 1, 2, 3])))),
            Some(Box::new(ListNode::from(vec![2, 3])))
        );
    }
}
