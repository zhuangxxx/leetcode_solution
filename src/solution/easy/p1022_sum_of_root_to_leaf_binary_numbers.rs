use crate::structure::tree_node::TreeNode;

struct Solution;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [1022. 从根到叶的二进制数之和](https://leetcode.cn/problems/sum-of-root-to-leaf-binary-numbers/)
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: i32) -> i32 {
            if let Some(root) = root {
                let prev = (prev << 1) + root.borrow().val;
                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    prev
                } else {
                    dfs(root.borrow().left.clone(), prev) + dfs(root.borrow().right.clone(), prev)
                }
            } else {
                0
            }
        }

        dfs(root, 0)
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 0, 1, 0, 1, 0, 1]".to_string()
            ))))),
            22
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
            0
        );
    }
}
