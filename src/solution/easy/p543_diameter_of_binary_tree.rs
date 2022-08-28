use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [543. 二叉树的直径](https://leetcode.cn/problems/diameter-of-binary-tree/)
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            if let Some(root) = root {
                let (l, r) = (
                    dfs(root.borrow().left.clone(), max),
                    dfs(root.borrow().right.clone(), max),
                );
                *max = std::cmp::max(*max, l + r);

                std::cmp::max(l, r) + 1
            } else {
                0
            }
        }

        let mut max = 0;
        dfs(root, &mut max);

        max
        // 0ms/2.7MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, 5]".to_string()
            ))))),
            3
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, 5, N, N, 6, N, N, 7, 8, N, 9, N, 10]".to_string()
            ))))),
            7
        );
    }
}
