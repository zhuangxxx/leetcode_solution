use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [226. 翻转二叉树](https://leetcode.cn/problems/invert-binary-tree/)
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        stack.push(root.clone());

        while !stack.is_empty() {
            for _ in 0..stack.len() {
                if let Some(root) = stack.pop() {
                    if let Some(root) = root {
                        let invert = TreeNode {
                            val: root.borrow().val,
                            left: root.borrow().right.clone(),
                            right: root.borrow().left.clone(),
                        };
                        *root.borrow_mut() = invert;

                        stack.push(root.borrow().left.clone());
                        stack.push(root.borrow().right.clone());
                    }
                }
            }
        }

        root
        // 0ms/1.9MB
    }

    // pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //     if let Some(root) = root {
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             val: root.borrow().val,
    //             left: Self::invert_tree(root.borrow().right.clone()),
    //             right: Self::invert_tree(root.borrow().left.clone()),
    //         })))
    //     } else {
    //         None
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::invert_tree(Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 7, 1, 3, 6, 9]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 7, 2, 9, 6, 3, 1]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::invert_tree(Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 1, 3]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 3, 1]".to_string()
            ))))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::invert_tree(None), None);
    }
}
