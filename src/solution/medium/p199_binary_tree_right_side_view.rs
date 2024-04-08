use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [199. 二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/)
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let n = queue.len() - 1;
            for i in 0..=n {
                if let Some(Some(node)) = queue.pop_front() {
                    if i == n {
                        view.push(node.borrow().val);
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

        view
        // 0ms/2.14MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, N, 5, N, 4]".to_string()
            ))))),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 3]".to_string()
            ))))),
            vec![1, 3]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::right_side_view(None), Vec::new());
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, 3, N, N, 4]".to_string()
            ))))),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::right_side_view(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, N, N, N, 5]".to_string()
            ))))),
            vec![1, 3, 4, 5]
        );
    }
}
