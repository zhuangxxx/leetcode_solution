use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [513. 找树左下角的值](https://leetcode.cn/problems/find-bottom-left-tree-value/)
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut bl = 0;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            for i in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    if i == 0 {
                        bl = node.borrow().val;
                    }

                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow_mut().left.take());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow_mut().right.take());
                    }
                }
            }
        }

        bl
        // 3ms/2.94MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 1, 3]".to_string()
            ))))),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, N, 5, 6, N, N, 7]".to_string()
            ))))),
            7
        );
    }
}
