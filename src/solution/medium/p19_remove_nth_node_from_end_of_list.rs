use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [19. 删除链表的倒数第 N 个结点](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/)
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let (mut fast, mut slow): (*const _, _) = (&head, head.as_mut().unwrap());
        let mut l = 0;
        while let Some(fp) = unsafe { &*fast } {
            fast = &fp.next;
            if l > n {
                slow = slow.next.as_mut().unwrap()
            }
            l += 1;
        }

        if l == n {
            head.unwrap().next
        } else {
            slow.next = slow.next.take().unwrap().next;
            head
        }
        // 0ms/2MB
    }

    // pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     let mut fast = &head;
    //     for _ in 0..n {
    //         if let Some(fp) = fast {
    //             fast = &fp.next;
    //         } else {
    //             return None;
    //         }
    //     }

    //     let mut list = ListNode::new(0);
    //     let mut tail = &mut list;

    //     let mut slow = &head;
    //     while let (Some(fp), Some(sp)) = (fast, slow) {
    //         fast = &fp.next;
    //         slow = &sp.next;

    //         tail.next = Some(Box::new(ListNode::new(sp.val)));
    //         tail = tail.next.as_mut().unwrap();
    //     }
    //     tail.next = slow.as_ref().unwrap().next.clone();

    //     list.next
    //     // 0ms/2MB
    // }

    // pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     fn dfs(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
    //         if let Some(ptr) = head {
    //             let (head, d) = dfs(ptr.next, n);
    //             (
    //                 Some(Box::new(ListNode {
    //                     val: ptr.val,
    //                     next: if n == d {
    //                         if let Some(ptr) = head {
    //                             ptr.next
    //                         } else {
    //                             None
    //                         }
    //                     } else {
    //                         head
    //                     },
    //                 })),
    //                 d + 1,
    //             )
    //         } else {
    //             (None, 0)
    //         }
    //     }

    //     let (head, d) = dfs(head, n);
    //     if d > n {
    //         head
    //     } else {
    //         head.unwrap().next
    //     }
    //     // 0ms/1.9MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(ListNode::from(vec![1, 2, 3, 4, 5]))), 2),
            Some(Box::new(ListNode::from(vec![1, 2, 3, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(ListNode::from(vec![1]))), 1),
            None
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(ListNode::from(vec![1, 2]))), 1),
            Some(Box::new(ListNode::from(vec![1])))
        );
    }
}
