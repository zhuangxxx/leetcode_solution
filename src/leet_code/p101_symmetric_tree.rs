use crate::data_struct::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [101. 对称二叉树](https://leetcode.cn/problems/symmetric-tree/)
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.borrow().left.clone());
            queue.push_back(root.borrow().right.clone());

            while !queue.is_empty() {
                if let (Some(p), Some(q)) = (queue.pop_front(), queue.pop_front()) {
                    if p.is_none() && q.is_none() {
                        continue;
                    }

                    if p.is_none() || q.is_none() {
                        return false;
                    }

                    if let (Some(p), Some(q)) = (p, q) {
                        if p.borrow().val != q.borrow().val {
                            return false;
                        }

                        queue.push_back(p.borrow().left.clone());
                        queue.push_back(q.borrow().right.clone());

                        queue.push_back(p.borrow().right.clone());
                        queue.push_back(q.borrow().left.clone());
                    }
                }
            }
        }

        true
        // 0ms/2.1MB
    }

    // fn is_symmetric_node(
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> bool {
    //     match (p, q) {
    //         (None, None) => true,
    //         (Some(_), None) | (None, Some(_)) => false,
    //         (Some(p), Some(q)) => {
    //             p.borrow().val == q.borrow().val
    //                 && Self::is_symmetric_node(p.borrow().left.clone(), q.borrow().right.clone())
    //                 && Self::is_symmetric_node(p.borrow().right.clone(), q.borrow().left.clone())
    //         }
    //     }
    // }

    // pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     if let Some(root) = root {
    //         Self::is_symmetric_node(root.borrow().left.clone(), root.borrow().right.clone())
    //     } else {
    //         true
    //     }
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(
            TreeNode::from("[1, 2, 2, 3, 4, 4, 3]".to_string())
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_symmetric(Some(Rc::new(RefCell::new(
            TreeNode::from("[1, 2, 2, N, 3, N, 3]".to_string())
        )))));
    }
}
