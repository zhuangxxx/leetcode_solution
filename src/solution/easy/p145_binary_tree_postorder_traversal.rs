use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [145. 二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/)
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut list = Vec::new();
        let (mut stack, mut prev) = (Vec::new(), None);

        let mut root = root;
        loop {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone();
            }

            if stack.is_empty() {
                break;
            }

            if let Some(node) = stack.pop() {
                if node.borrow().right.is_none() || node.borrow().right == prev {
                    list.push(node.borrow().val);
                    prev = Some(node.clone());
                    root = None;
                } else {
                    stack.push(node.clone());
                    root = node.borrow().right.clone();
                }
            }
        }

        list
        // 0ms/2MB
    }

    // pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut list = Vec::new();

    //     if let Some(root) = root {
    //         list.append(&mut Self::postorder_traversal(root.borrow().left.clone()));
    //         list.append(&mut Self::postorder_traversal(root.borrow().right.clone()));
    //         list.push(root.borrow().val);
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
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, 3]".to_string()
            ))))),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::postorder_traversal(None), vec![]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
