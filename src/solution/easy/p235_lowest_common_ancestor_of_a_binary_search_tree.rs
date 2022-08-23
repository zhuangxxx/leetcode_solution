use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [235. 二叉搜索树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/)
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(p), Some(q)) = (p, q) {
            let mut root = root;
            while let Some(node) = root.clone() {
                if node.borrow().val > p.borrow().val && node.borrow().val > q.borrow().val {
                    root = node.borrow().left.clone();
                } else if node.borrow().val < p.borrow().val && node.borrow().val < q.borrow().val {
                    root = node.borrow().right.clone();
                } else {
                    break;
                }
            }
            root
        } else {
            root
        }
        // 4ms/3.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[6, 2, 8, 0, 4, 7, 9, N, N, 3, 5]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[2, 0, 4, N, N, 3, 5]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[8, 7, 9]".to_string()
                ))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[6, 2, 8, 0, 4, 7, 9, N, N, 3, 5]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[6, 2, 8, 0, 4, 7, 9, N, N, 3, 5]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[2, 0, 4, N, N, 3, 5]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[4, 3, 5]".to_string()
                ))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 0, 4, N, N, 3, 5]".to_string()
            ))))
        );
    }
}
