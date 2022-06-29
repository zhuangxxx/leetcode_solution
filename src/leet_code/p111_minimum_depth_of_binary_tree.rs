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
    /// [111. 二叉树的最小深度](https://leetcode.cn/problems/minimum-depth-of-binary-tree/)
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((root, 1));

            let mut depth = 1;
            while !queue.is_empty() {
                if let Some((root, depth)) = queue.pop_front() {
                    if root.borrow().left.is_none() && root.borrow().right.is_none() {
                        return depth;
                    }

                    if let Some(left) = root.borrow().left.clone() {
                        queue.push_back((left, depth + 1));
                    }
                    if let Some(right) = root.borrow().right.clone() {
                        queue.push_back((right, depth + 1));
                    }
                }
            }

            depth
        } else {
            0
        }
        // 36ms/12.7MB
    }

    // pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if let Some(root) = root {
    //         match (root.borrow().left.clone(), root.borrow().right.clone()) {
    //             (None, None) => 1,
    //             (node, None) | (None, node) => Self::min_depth(node) + 1,
    //             (left, right) => std::cmp::min(Self::min_depth(left), Self::min_depth(right)) + 1,
    //         }
    //     } else {
    //         0
    //     }
    //     // 48ms/13.4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            })))),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                        })))
                    })))
                })))
            })))),
            5
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                })))
            })))),
            3
        );
    }
}
