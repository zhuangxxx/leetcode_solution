use crate::data_struct::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [104. 二叉树的最大深度](https://leetcode.cn/problems/maximum-depth-of-binary-tree/)
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;

        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);

            while !queue.is_empty() {
                let mut len = queue.len();
                while len > 0 {
                    if let Some(root) = queue.pop_front() {
                        if let Some(left) = root.borrow().left.clone() {
                            queue.push_back(left);
                        }

                        if let Some(right) = root.borrow().right.clone() {
                            queue.push_back(right);
                        }
                    }

                    len -= 1;
                }

                depth += 1;
            }
        }

        depth
        // 0ms/2.5MB
    }

    // pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if let Some(root) = root {
    //         std::cmp::max(
    //             Self::max_depth(root.borrow().left.clone()),
    //             Self::max_depth(root.borrow().right.clone()),
    //         ) + 1
    //     } else {
    //         0
    //     }
    //     // 0ms/2.6MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 3, 20, N, N, 15, 7]".to_string()
            ))))),
            3
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, N, N, 5]".to_string()
            ))))),
            3
        );
    }
}
