use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [700. 二叉搜索树中的搜索](https://leetcode.cn/problems/search-in-a-binary-search-tree/)
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = root.clone() {
            if node.borrow().val > val {
                root = node.borrow().left.clone();
            } else if node.borrow().val < val {
                root = node.borrow().right.clone();
            } else {
                break;
            }
        }

        root
        // 4ms/2.8MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::search_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[4, 2, 7, 1, 3]".to_string()
                )))),
                2
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 1, 3]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::search_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[4, 2, 7, 1, 3]".to_string()
                )))),
                5
            ),
            None
        );
    }
}
