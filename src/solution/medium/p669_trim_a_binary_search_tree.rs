use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [669. 修剪二叉搜索树](https://leetcode.cn/problems/trim-a-binary-search-tree/)
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let val = node.borrow().val;
            if val < low {
                return Self::trim_bst(node.borrow_mut().right.take(), low, high);
            } else {
                let left = Self::trim_bst(node.borrow().left.clone(), low, high);
                node.borrow_mut().left = left;
            }
            if val > high {
                return Self::trim_bst(node.borrow_mut().left.take(), low, high);
            } else {
                let right = Self::trim_bst(node.borrow().right.clone(), low, high);
                node.borrow_mut().right = right;
            }

            Some(node)
        } else {
            None
        }
        // 0ms/3.09MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::trim_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, 0, 2]".to_string()
                )))),
                1,
                2
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::trim_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[3, 0, 4, N, 2, N, N, 1]".to_string()
                )))),
                1,
                3
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 2, N, 1]".to_string()
            ))))
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::trim_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 3, 7, 1, 4, 6, 8]".to_string()
                )))),
                6,
                8
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[7, 6, 8]".to_string()
            ))))
        );
    }
}
