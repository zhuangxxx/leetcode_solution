use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [98. 验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/)
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (mut list, mut stack) = (Vec::new(), Vec::new());
        let mut root = root;

        loop {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone()
            }

            if stack.is_empty() {
                break;
            }

            if let Some(node) = stack.pop() {
                list.push(node.borrow().val);
                root = node.borrow().right.clone();
            }
        }

        list.windows(2).all(|w| w[0] < w[1])
        // 0ms/2.95MB
    }

    // pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     fn dfs(root: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
    //         if let Some(root) = root {
    //             if min.is_some() && root.borrow().val <= min.unwrap()
    //                 || (max.is_some() && root.borrow().val >= max.unwrap())
    //             {
    //                 false
    //             } else {
    //                 dfs(root.borrow().left.clone(), min, Some(root.borrow().val))
    //                     && dfs(root.borrow().right.clone(), Some(root.borrow().val), max)
    //             }
    //         } else {
    //             true
    //         }
    //     }

    //     dfs(root, None, None)
    //     // 0ms/2.85MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::from("[2, 1, 3]".to_string())
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::from("[5, 1, 4, N, N, 3, 6]".to_string())
        )))));
    }

    #[test]
    fn trap1() {
        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::from("[4, 2, 5, 1, 3]".to_string())
        )))));
    }

    #[test]
    fn fail1() {
        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::from("[2147483647]".to_string())
        )))));
    }

    #[test]
    fn fail2() {
        assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(
            TreeNode::from("[-2147483648, N, 2147483647]".to_string())
        )))));
    }
}
