use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [404. 左叶子之和](https://leetcode.cn/problems/sum-of-left-leaves/)
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((root.borrow().left.clone(), root.borrow().right.clone()));

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some((left, right)) = queue.pop_front() {
                        if let Some(left) = left {
                            if left.borrow().left.is_none() && left.borrow().right.is_none() {
                                sum += left.borrow().val;
                            } else {
                                queue.push_back((
                                    left.borrow().left.clone(),
                                    left.borrow().right.clone(),
                                ));
                            }
                        }

                        if let Some(right) = right {
                            if right.borrow().left.is_some() || right.borrow().right.is_some() {
                                queue.push_back((
                                    right.borrow().left.clone(),
                                    right.borrow().right.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        sum
        // 0ms/2.1MB
    }

    // pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     fn sum_left(
    //         left: Option<Rc<RefCell<TreeNode>>>,
    //         right: Option<Rc<RefCell<TreeNode>>>,
    //     ) -> i32 {
    //         let mut sum = 0;

    //         if let Some(left) = left {
    //             if left.borrow().left.is_none() && left.borrow().right.is_none() {
    //                 sum += left.borrow().val;
    //             } else {
    //                 sum += sum_left(left.borrow().left.clone(), left.borrow().right.clone());
    //             }
    //         }

    //         if let Some(right) = right {
    //             if right.borrow().left.is_some() || right.borrow().right.is_some() {
    //                 sum += sum_left(right.borrow().left.clone(), right.borrow().right.clone());
    //             }
    //         }

    //         sum
    //     }

    //     if let Some(root) = root {
    //         sum_left(root.borrow().left.clone(), root.borrow().right.clone())
    //     } else {
    //         0
    //     }
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))),
            24
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            0
        );
    }
}
