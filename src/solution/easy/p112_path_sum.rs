use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [112. 路径总和](https://leetcode.cn/problems/path-sum/)
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((root, target_sum));

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some((root, target_sum)) = queue.pop_front() {
                        if root.borrow().left.is_none() && root.borrow().right.is_none() {
                            if root.borrow().val == target_sum {
                                return true;
                            }
                            continue;
                        }

                        if let Some(left) = root.borrow().left.clone() {
                            queue.push_back((left, target_sum - root.borrow().val));
                        }
                        if let Some(right) = root.borrow().right.clone() {
                            queue.push_back((right, target_sum - root.borrow().val));
                        }
                    }
                }
            }
        }

        false
        // 0ms/2.3MB
    }

    // pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    //     if let Some(root) = root {
    //         match (root.borrow().left.clone(), root.borrow().right.clone()) {
    //             (None, None) => target_sum == root.borrow().val,
    //             (node, None) | (None, node) => {
    //                 Self::has_path_sum(node, target_sum - root.borrow().val)
    //             }
    //             (left, right) => {
    //                 Self::has_path_sum(left, target_sum - root.borrow().val)
    //                     || Self::has_path_sum(right, target_sum - root.borrow().val)
    //             }
    //         }
    //     } else {
    //         false
    //     }
    //     // 0ms/2.7MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 4, 8, 11, N, 8, 4, 7, 2, N, N, N, 1]".to_string()
            )))),
            22
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 2]".to_string()
            )))),
            5
        ));
    }
}
