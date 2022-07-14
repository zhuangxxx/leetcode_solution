use crate::data_struct::list_node::ListNode;

struct Solution;

impl Solution {
    /// [234. 回文链表](https://leetcode.cn/problems/palindrome-linked-list/)
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        fn reverse(
            prev: Option<Box<ListNode>>,
            mut tail: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            if tail.is_none() {
                return prev;
            }

            let next = std::mem::replace(
                std::borrow::BorrowMut::borrow_mut(&mut tail.as_mut().unwrap().next),
                prev,
            );

            reverse(tail, next)
        }

        pub fn compare(head: Option<Box<ListNode>>, rev: Option<Box<ListNode>>) -> bool {
            match (head, rev) {
                (None, None) | (None, Some(_)) | (Some(_), None) => true,
                (Some(h), Some(r)) => {
                    if h.val == r.val {
                        compare(h.next, r.next)
                    } else {
                        false
                    }
                }
            }
        }

        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true;
        }

        let (mut fast, mut slow) = (&head, &head);
        while fast.is_some() {
            fast = fast
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .map(|x| &x.next)
                .unwrap_or(&None);
            slow = slow.as_ref().map(|x| &x.next).unwrap();
        }

        let s = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let slow = unsafe { (*s).take() };
        let rev = reverse(None, slow);

        compare(head, rev)
        // 48ms/8MB
    }

    // pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    //     fn recursive(head: &Option<Box<ListNode>>, tail: &mut &Option<Box<ListNode>>) -> bool {
    //         if let Some(h) = head {
    //             let result = recursive(&h.next, tail);

    //             if let Some(t) = tail {
    //                 if h.val != t.val {
    //                     return false;
    //                 }

    //                 *tail = &t.next;
    //             }

    //             result
    //         } else {
    //             true
    //         }
    //     }

    //     let mut tail = &head;

    //     recursive(&head, &mut tail)
    //     // 60ms/8.9MB
    // }

    // pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    //     if let Some(head) = head {
    //         let (mut tail, mut values) = (head.as_ref(), vec![head.val]);
    //         while let Some(node) = tail.next.as_ref() {
    //             values.push(node.val);
    //             tail = node.as_ref();
    //         }

    //         tail = head.as_ref();
    //         while let Some(node) = tail.next.as_ref() {
    //             if values.is_empty() || values.pop().unwrap() != tail.val {
    //                 return false;
    //             }

    //             tail = node.as_ref();
    //         }
    //     }

    //     true
    //     // 48ms/9MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(Some(Box::new(ListNode::from(
            vec![1, 2, 2, 1]
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome(Some(Box::new(ListNode::from(
            vec![1, 2]
        )))));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_palindrome(Some(Box::new(ListNode::from(
            vec![1, 1, 2, 1]
        )))));
    }
}
