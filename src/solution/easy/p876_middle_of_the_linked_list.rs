use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [876. 链表的中间结点](https://leetcode.cn/problems/middle-of-the-linked-list/)
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (head.clone(), head);
        while let Some(tail) = slow {
            for _ in 0..2 {
                if let Some(node) = fast {
                    fast = node.next;
                } else {
                    return Some(tail);
                }
            }
            slow = tail.next;
        }

        slow
        // 0ms/2MB
    }

    // pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let (mut fast, mut slow, mut next) = (head.clone(), head, false);
    //     while let Some(tail) = fast {
    //         fast = tail.next;
    //         if next {
    //             if let Some(tail) = slow {
    //                 slow = tail.next;
    //             }
    //         }
    //         next = !next;
    //     }

    //     slow
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::middle_node(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5])))),
            Some(Box::new(ListNode::from(vec![3, 4, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::middle_node(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5, 6])))),
            Some(Box::new(ListNode::from(vec![4, 5, 6])))
        );
    }

    #[test]
    fn test() {
        let list = Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5, 6])));
        let mut n = true;
        let (mut fast, mut slow) = (list.clone(), list);
        while let Some(tail) = fast.take() {
            println!("fast:{}", tail.val);
            fast = tail.next;
            if n {
                if let Some(tail) = slow.take() {
                    println!("slow:{}", tail.val);
                    slow = tail.next;
                }
            }
            n = !n;
        }
    }
}
