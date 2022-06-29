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
    /// [110. 平衡二叉树](https://leetcode.cn/problems/balanced-binary-tree/)
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let (left_height, right_height) = (
                    height(root.borrow().left.clone()),
                    height(root.borrow().right.clone()),
                );

                if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1
                {
                    -1
                } else {
                    std::cmp::max(left_height, right_height) + 1
                }
            } else {
                0
            }
        }

        height(root) != -1
        // 0ms/2.8MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_balanced(Some(Rc::new(RefCell::new(
            TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            }
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_balanced(Some(Rc::new(RefCell::new(
            TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
            }
        )))));
    }
}
