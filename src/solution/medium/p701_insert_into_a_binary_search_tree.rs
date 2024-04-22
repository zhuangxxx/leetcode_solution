use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [701. 二叉搜索树中的插入操作](https://leetcode.cn/problems/insert-into-a-binary-search-tree/)
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let new = Rc::new(RefCell::new(TreeNode::new(val)));
        if let Some(node) = root.clone() {
            let (mut temp, mut prev) = (root.clone(), node.borrow().val);
            while let Some(node) = temp {
                if val < node.borrow().val {
                    if node.borrow().left.is_none() {
                        node.borrow_mut().left = Some(new);
                        break;
                    }
                    temp = node.borrow().left.clone();
                } else {
                    if node.borrow().right.is_none() {
                        node.borrow_mut().right = Some(new);
                        break;
                    }
                    temp = node.borrow().right.clone();
                }
            }

            root
        } else {
            Some(new)
        }
        // 9ms/2.59MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::insert_into_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[4, 2, 7, 1, 3]".to_string()
                )))),
                5
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 7, 1, 3, 5]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::insert_into_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[40, 20, 60, 10, 30, 50, 70]".to_string()
                )))),
                25
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[40, 20, 60, 10, 30, 50, 70, N, N, 25]".to_string()
            ))))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::insert_into_bst(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[4, 2, 7, 1, 3, N, N, N, N, N, N]".to_string()
                )))),
                5
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 7, 1, 3, 5]".to_string()
            ))))
        );
    }
}
