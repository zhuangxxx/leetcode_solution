use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [129. 求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/)
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
            if let Some(root) = root {
                let sum = sum * 10 + root.borrow().val;
                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    sum
                } else {
                    dfs(root.borrow().left.clone(), sum) + dfs(root.borrow().right.clone(), sum)
                }
            } else {
                0
            }
        }

        dfs(root, 0)
        // 0ms/2.06MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            ))))),
            25
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sum_numbers(Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 9, 0, 5, 1]".to_string()
            ))))),
            1026
        );
    }
}
