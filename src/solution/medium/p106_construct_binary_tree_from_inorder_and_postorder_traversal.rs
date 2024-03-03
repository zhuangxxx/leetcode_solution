use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [106. 从中序与后序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            postorder[postorder.len() - 1],
        ))));
        let (mut stack, mut index) = (vec![root.clone()], inorder.len() - 1);
        for val in postorder.into_iter().rev().skip(1) {
            if let Some(Some(node)) = stack.last() {
                let mut node = node.clone();
                if node.borrow().val != inorder[index] {
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    stack.push(node.borrow().right.clone());
                } else {
                    while let Some(Some(last)) = stack.last() {
                        if last.borrow().val != inorder[index] {
                            break;
                        }
                        node = last.clone();
                        stack.pop();
                        index -= 1;
                    }
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    stack.push(node.borrow().left.clone());
                }
            }
        }

        root
        // 0ms/2.34MB
    }

    // pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn dfs(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    //         if postorder.is_empty() {
    //             return None;
    //         }

    //         let root = Rc::new(RefCell::new(TreeNode::new(postorder[postorder.len() - 1])));
    //         if let Some(mid) = inorder
    //             .iter()
    //             .position(|&val| val == postorder[postorder.len() - 1])
    //         {
    //             if mid > 0 {
    //                 root.borrow_mut().left = dfs(&inorder[0..mid], &postorder[0..mid]);
    //             }
    //             if mid < inorder.len() - 1 {
    //                 root.borrow_mut().right =
    //                     dfs(&inorder[mid + 1..], &postorder[mid..postorder.len() - 1]);
    //             }
    //         }

    //         Some(root)
    //     }

    //     dfs(&inorder, &postorder)
    //     // 3ms/2.63MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            Some(Rc::new(RefCell::new(TreeNode::new(-1))))
        );
    }
}
