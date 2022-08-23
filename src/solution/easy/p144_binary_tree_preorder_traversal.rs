use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [144. 二叉树的前序遍历](https://leetcode.cn/problems/binary-tree-preorder-traversal/)
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut list = Vec::new();
        let mut stack = Vec::new();

        let mut root = root;
        loop {
            while let Some(node) = root {
                list.push(node.borrow().val);
                stack.push(node.clone());
                root = node.borrow().left.clone()
            }

            if stack.is_empty() {
                break;
            }

            if let Some(node) = stack.pop() {
                root = node.borrow().right.clone();
            }
        }

        list
        // 0ms/2MB
    }

    // pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut list = Vec::new();

    //     if let Some(root) = root {
    //         list.push(root.borrow().val);
    //         list.append(&mut Self::preorder_traversal(root.borrow().left.clone()));
    //         list.append(&mut Self::preorder_traversal(root.borrow().right.clone()));
    //     }

    //     list
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, 3]".to_string()
            ))))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::preorder_traversal(None), vec![]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2]".to_string()
            ))))),
            vec![1, 2]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2]".to_string()
            ))))),
            vec![1, 2]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::preorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 4, 3, 2]".to_string()
            ))))),
            vec![1, 4, 2, 3]
        );
    }
}
