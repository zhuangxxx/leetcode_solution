use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [103. 二叉树的锯齿形层序遍历](https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/)
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut order = Vec::new();

        let (mut even, mut odd) = (Vec::new(), Vec::new());
        even.push((0, root));

        while !even.is_empty() || !odd.is_empty() {
            while let Some((level, root)) = even.pop() {
                if let Some(root) = root {
                    if order.len() == level {
                        order.push(Vec::new());
                    }
                    order[level].push(root.borrow().val);

                    if root.borrow().left.is_some() {
                        odd.push((level + 1, root.borrow().left.clone()));
                    }
                    if root.borrow().right.is_some() {
                        odd.push((level + 1, root.borrow().right.clone()));
                    }
                }
            }
            while let Some((level, root)) = odd.pop() {
                if let Some(root) = root {
                    if order.len() == level {
                        order.push(Vec::new());
                    }
                    order[level].push(root.borrow().val);

                    if root.borrow().right.is_some() {
                        even.push((level + 1, root.borrow().right.clone()));
                    }
                    if root.borrow().left.is_some() {
                        even.push((level + 1, root.borrow().left.clone()));
                    }
                }
            }
        }

        order
        // 0ms/2.03MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![vec![1]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::zigzag_level_order(None), Vec::<Vec<_>>::new());
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::zigzag_level_order(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]".to_string()
            ))))),
            vec![
                vec![1],
                vec![3, 2],
                vec![4, 5, 6, 7],
                vec![15, 14, 13, 12, 11, 10, 9, 8]
            ]
        );
    }
}
