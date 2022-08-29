use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [572. 另一棵树的子树](https://leetcode.cn/problems/subtree-of-another-tree/)
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn check(
            root: Option<Rc<RefCell<TreeNode>>>,
            sub_root: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if root.is_none() && sub_root.is_none() {
                return true;
            }

            if let (Some(root), Some(sub_root)) = (root, sub_root) {
                root.borrow().val == sub_root.borrow().val
                    && check(root.borrow().left.clone(), sub_root.borrow().left.clone())
                    && check(root.borrow().right.clone(), sub_root.borrow().right.clone())
            } else {
                false
            }
        }

        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            sub_root: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if check(root.clone(), sub_root.clone()) {
                return true;
            }

            if let Some(root) = root {
                dfs(root.borrow().left.clone(), sub_root.clone())
                    || dfs(root.borrow().right.clone(), sub_root)
            } else {
                false
            }
        }

        dfs(root, sub_root)
        // 8ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_subtree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 4, 5, 1, 2]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 1, 2]".to_string()
            ))))
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_subtree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 4, 5, 1, 2, N, N, N, N, 0]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 1, 2]".to_string()
            ))))
        ));
    }

    #[test]
    fn fail1() {
        assert!(Solution::is_subtree(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 1, N, 1, N, 1, N, 1, N, 1, N, 1, N, 1, N, 1, N, 1, N, 1, 2]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 1, N, 1, N, 1, N, 1, N, 1, 2]".to_string()
            ))))
        ));
    }
}
