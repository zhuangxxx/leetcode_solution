// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [94. 二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/)
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

        list
        // 0ms/1.9MB
    }

    // pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut list = Vec::new();

    //     if let Some(root) = root {
    //         let node = root.borrow();

    //         list.append(&mut Self::inorder_traversal(node.left.clone()));
    //         list.push(node.val);
    //         list.append(&mut Self::inorder_traversal(node.right.clone()));
    //     }

    //     list
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None
                })))
            })))),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::inorder_traversal(None), Vec::new());
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
