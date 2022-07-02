use crate::data_struct::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [110. 平衡二叉树](https://leetcode.cn/problems/balanced-binary-tree/)
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let (left_height, right_height) = (
                    height(root.borrow().left.clone()),
                    height(root.borrow().right.clone()),
                );

                if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1
                {
                    -1
                } else {
                    std::cmp::max(left_height, right_height) + 1
                }
            } else {
                0
            }
        }

        height(root) != -1
        // 0ms/2.8MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(
            TreeNode::from("[3, 9, 20, N, N, 15, 7]".to_string())
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_balanced(Some(Rc::new(RefCell::new(
            TreeNode::from("[1, 2, 2, 3, 3, N, N, 4, 4]".to_string())
        )))));
    }
}
