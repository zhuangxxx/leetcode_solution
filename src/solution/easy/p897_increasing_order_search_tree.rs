use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [897. 递增顺序搜索树](https://leetcode.cn/problems/increasing-order-search-tree/)
    pub fn increasing_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut tree, mut tail): (_, Option<Rc<RefCell<TreeNode>>>) = (None, None);
        let mut stack = Vec::new();
        loop {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone();
            }

            if stack.is_empty() {
                break;
            }

            if let Some(node) = stack.pop() {
                let temp = Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
                if let Some(ptr) = tail {
                    ptr.borrow_mut().right = temp.clone();
                    tail = temp;
                } else {
                    tree = temp;
                    tail = tree.clone();
                }
                root = node.borrow().right.clone();
            }
        }

        tree
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::increasing_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 3, 6, 2, 4, N, 8, 1, N, N, N, 7, 9]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, N, 3, N, 4, N, 5, N, 6, N, 7, N, 8, N, 9]".to_string()
            ))))
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::increasing_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 1, 7]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 5, N, 7]".to_string()
            ))))
        )
    }
}
