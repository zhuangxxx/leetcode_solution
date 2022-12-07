use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [872. 叶子相似的树](https://leetcode.cn/problems/leaf-similar-trees/)
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut root = root1;
        let mut leaf = Vec::new();
        let mut stack = Vec::new();
        loop {
            while let Some(node) = root {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    leaf.push(node.borrow().val);
                }
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
        leaf.reverse();

        root = root2;
        loop {
            while let Some(node) = root {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    if let Some(val) = leaf.pop() {
                        if node.borrow().val != val {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
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

        leaf.is_empty()
        // 0ms/2.1MB
    }

    // pub fn leaf_similar(
    //     root1: Option<Rc<RefCell<TreeNode>>>,
    //     root2: Option<Rc<RefCell<TreeNode>>>,
    // ) -> bool {
    //     let mut leaf = Vec::new();

    //     let mut stack = Vec::new();
    //     stack.push(root1);
    //     while !stack.is_empty() {
    //         if let Some(Some(root)) = stack.pop() {
    //             if root.borrow().left.is_none() && root.borrow().right.is_none() {
    //                 leaf.push(root.borrow().val);
    //             }
    //             if root.borrow().left.is_some() {
    //                 stack.push(root.borrow().left.clone());
    //             }
    //             if root.borrow().right.is_some() {
    //                 stack.push(root.borrow().right.clone());
    //             }
    //         }
    //     }
    //     leaf.reverse();

    //     stack.push(root2);
    //     while !stack.is_empty() {
    //         if let Some(Some(root)) = stack.pop() {
    //             if root.borrow().left.is_none() && root.borrow().right.is_none() {
    //                 if let Some(val) = leaf.pop() {
    //                     if root.borrow().val != val {
    //                         return false;
    //                     }
    //                 } else {
    //                     return false;
    //                 }
    //             }
    //             if root.borrow().left.is_some() {
    //                 stack.push(root.borrow().left.clone());
    //             }
    //             if root.borrow().right.is_some() {
    //                 stack.push(root.borrow().right.clone());
    //             }
    //         }
    //     }

    //     leaf.is_empty()
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 5, 1, 6, 2, 9, 8, N, N, 7, 4]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 5, 1, 6, 7, 4, 2, N, N, N, N, N, N, 9, 8]".to_string()
            ))))
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 3, 2]".to_string()
            ))))
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 6, 1, 3, 5, 7]".to_string()
            )))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 6, N, 3, 5, 7]".to_string()
            ))))
        ));
    }
}
