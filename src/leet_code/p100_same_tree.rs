use crate::data_struct::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [100. 相同的树](https://leetcode.cn/problems/same-tree/)
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(p);
        queue.push_back(q);

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
                    queue.push_back(q.borrow().left.clone());

                    queue.push_back(p.borrow().right.clone());
                    queue.push_back(q.borrow().right.clone());
                }
            }
        }

        true
        // 0ms/2.1MB
    }

    // pub fn is_same_tree(
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> bool {
    //     match (p, q) {
    //         (None, None) => true,
    //         (None, Some(_)) | (Some(_), None) => false,
    //         (Some(p), Some(q)) => {
    //             let (pb, qb) = (p.borrow(), q.borrow());
    //             if pb.val != qb.val {
    //                 return false;
    //             }
    //             Self::is_same_tree(pb.left.clone(), qb.left.clone())
    //                 && Self::is_same_tree(pb.right.clone(), qb.right.clone())
    //         }
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            ))))
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, N]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2]".to_string()
            ))))
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 1]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 1, 2]".to_string()
            ))))
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2]".to_string()
            ))))
        ));
    }
}
