use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [617. 合并二叉树](https://leetcode.cn/problems/merge-two-binary-trees/)
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(root1), Some(root2)) => Some(Rc::new(RefCell::new(TreeNode {
                val: root1.borrow().val + root2.borrow().val,
                left: Solution::merge_trees(
                    root1.borrow().left.clone(),
                    root2.borrow().left.clone(),
                ),
                right: Solution::merge_trees(
                    root1.borrow().right.clone(),
                    root2.borrow().right.clone(),
                ),
            }))),
            (root @ Some(_), None) | (None, root @ Some(_)) => root,
            (None, None) => None,
        }
        // 4ms/2.3MB
    }

    // pub fn merge_trees(
    //     root1: Option<Rc<RefCell<TreeNode>>>,
    //     root2: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     match (root1, root2) {
    //         (Some(root1), Some(root2)) => Some(Rc::new(RefCell::new(TreeNode {
    //             val: root1.borrow().val + root2.borrow().val,
    //             left: Solution::merge_trees(
    //                 root1.borrow().left.clone(),
    //                 root2.borrow().left.clone(),
    //             ),
    //             right: Solution::merge_trees(
    //                 root1.borrow().right.clone(),
    //                 root2.borrow().right.clone(),
    //             ),
    //         }))),
    //         (Some(root1), None) => Some(Rc::new(RefCell::new(TreeNode {
    //             val: root1.borrow().val,
    //             left: Solution::merge_trees(root1.borrow().left.clone(), None),
    //             right: Solution::merge_trees(root1.borrow().right.clone(), None),
    //         }))),
    //         (None, Some(root2)) => Some(Rc::new(RefCell::new(TreeNode {
    //             val: root2.borrow().val,
    //             left: Solution::merge_trees(None, root2.borrow().left.clone()),
    //             right: Solution::merge_trees(None, root2.borrow().right.clone()),
    //         }))),
    //         (None, None) => None,
    //     }
    //     // 8ms/2.5MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_trees(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, 3, 2, 5]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[2, 1, 3, N, 4, N, 7]".to_string()
                ))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 4, 5, 5, 4, N, 7]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge_trees(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode::from("[1, 2]".to_string()))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from("[2, 2]".to_string()))))
        );
    }
}
