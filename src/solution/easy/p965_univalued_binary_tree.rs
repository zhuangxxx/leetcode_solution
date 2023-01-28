use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [965. 单值二叉树](https://leetcode.cn/problems/univalued-binary-tree/)
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let target = root.borrow().val;
            let mut stack = vec![root];
            while let Some(root) = stack.pop() {
                if root.borrow().val != target {
                    return false;
                }

                if let Some(right) = root.borrow().right.clone() {
                    stack.push(right);
                }

                if let Some(left) = root.borrow().left.clone() {
                    stack.push(left);
                }
            }
        }

        true
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_unival_tree(Some(Rc::new(RefCell::new(
            TreeNode::from("[1, 1, 1, 1, 1, N, 1]".to_string())
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_unival_tree(Some(Rc::new(RefCell::new(
            TreeNode::from("[2, 2, 2, 5, 2]".to_string())
        )))));
    }
}
