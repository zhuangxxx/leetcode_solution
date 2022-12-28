use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [938. 二叉搜索树的范围和](https://leetcode.cn/problems/range-sum-of-bst/)
    pub fn range_sum_bst(mut root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;

        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some(root) = queue.pop_front() {
                        if root.borrow().val < low {
                            if let Some(right) = root.borrow().right.clone() {
                                queue.push_back(right);
                            }
                        } else if root.borrow().val > high {
                            if let Some(left) = root.borrow().left.clone() {
                                queue.push_back(left);
                            }
                        } else {
                            sum += root.borrow().val;
                            if let Some(right) = root.borrow().right.clone() {
                                queue.push_back(right);
                            }
                            if let Some(left) = root.borrow().left.clone() {
                                queue.push_back(left);
                            }
                        }
                    }
                }
            }
        }

        sum
        // 8ms/4MB
    }

    // pub fn range_sum_bst(mut root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    //     let mut sum = 0;

    //     let mut stack = Vec::new();
    //     loop {
    //         while let Some(node) = root {
    //             stack.push(node.clone());
    //             root = node.borrow().left.clone();

    //             if node.borrow().val < low {
    //                 break;
    //             }
    //         }

    //         if stack.is_empty() {
    //             break;
    //         }

    //         if let Some(node) = stack.pop() {
    //             root = node.borrow().right.clone();

    //             if node.borrow().val < low {
    //                 continue;
    //             } else if node.borrow().val > high {
    //                 break;
    //             } else {
    //                 sum += node.borrow().val;
    //             }
    //         }
    //     }

    //     sum
    //     // 12ms/4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::range_sum_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[10, 5, 15, 3, 7, N, 18]".to_string()
                )))),
                7,
                15
            ),
            32
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::range_sum_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[10, 5, 15, 3, 7, 13, 18, 1, N, 6]".to_string()
                )))),
                6,
                10
            ),
            23
        );
    }
}
