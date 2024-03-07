use crate::structure::list_node::ListNode;
use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [109. 有序链表转换二叉搜索树](https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/)
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            head: &mut Option<Box<ListNode>>,
            left: usize,
            right: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right {
                return None;
            }

            let mid = (left + right + 1) / 2;
            let mut left = if mid > 0 {
                dfs(head, left, mid - 1)
            } else {
                None
            };
            if let Some(tail) = head {
                let root = Rc::new(RefCell::new(TreeNode::new(tail.val)));
                root.borrow_mut().left = left.take();
                *head = tail.next.take();
                root.borrow_mut().right = dfs(head, mid + 1, right);
                Some(root)
            } else {
                None
            }
        }

        let len = {
            let mut len = 0;
            let mut head = &head;
            while let Some(tail) = head {
                len += 1;
                head = &tail.next;
            }

            len
        };
        dfs(&mut head, 0, len - 1)
        // 0ms/3.33MB
    }

    // pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn dfs(head: &Option<Box<ListNode>>, last: Option<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //         if let Some(tail) = head {
    //             let (mut slow, mut fast) = (tail, tail);
    //             if let Some(last) = last {
    //                 while fast.next.is_some()
    //                     && fast.next.as_ref().unwrap().val != last
    //                     && fast.next.as_ref().unwrap().next.is_some()
    //                     && fast.next.as_ref().unwrap().next.as_ref().unwrap().val != last
    //                 {
    //                     slow = slow.next.as_ref().unwrap();
    //                     fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
    //                 }
    //             } else {
    //                 while fast.next.is_some() && fast.next.as_ref().unwrap().next.is_some() {
    //                     slow = slow.next.as_ref().unwrap();
    //                     fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
    //                 }
    //             }
    //             let root = Rc::new(RefCell::new(TreeNode::new(slow.val)));
    //             if slow != tail {
    //                 root.borrow_mut().left = dfs(head, Some(slow.val));
    //             }
    //             if slow.next.is_some()
    //                 && (last.is_none() || slow.next.as_ref().unwrap().val != last.unwrap())
    //             {
    //                 root.borrow_mut().right = dfs(&slow.next, last);
    //             }

    //             Some(root)
    //         } else {
    //             None
    //         }
    //     }

    //     dfs(&head, None)
    //     // 120ms/3.94MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sorted_list_to_bst(Some(Box::new(ListNode::from(vec![-10, -3, 0, 5, 9])))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, -3, 9, -10, N, 5]".to_string()
            ))))
        );

        // assert_eq!(
        //     Solution::sorted_list_to_bst(Some(Box::new(ListNode::from(vec![-10, -3, 0, 5, 9])))),
        //     Some(Rc::new(RefCell::new(TreeNode::from(
        //         "[0, -10, 5, N, -3, N, 9]".to_string()
        //     ))))
        // );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::sorted_list_to_bst(None), None);
    }
}
