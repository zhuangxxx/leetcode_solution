use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [671. 二叉树中第二小的节点](https://leetcode.cn/problems/second-minimum-node-in-a-binary-tree/)
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut second = -1;

        if let Some(root) = root {
            let first = root.borrow().val;

            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root.borrow().left.clone());
            queue.push_back(root.borrow().right.clone());

            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some(root) = queue.pop_front() {
                        if let Some(root) = root {
                            if root.borrow().val > first
                                && (second == -1 || root.borrow().val < second)
                            {
                                second = root.borrow().val;
                            }

                            queue.push_back(root.borrow().left.clone());
                            queue.push_back(root.borrow().right.clone());
                        }
                    }
                }
            }
        }

        second
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 2, 5, N, N, 5, 7]".to_string()
            ))))),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_second_minimum_value(Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 2, 2]".to_string()
            ))))),
            -1
        );
    }
}
