use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [102. 二叉树的层序遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/)
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut order = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, root));

        while let Some((level, root)) = queue.pop_front() {
            if let Some(root) = root {
                if order.len() == level {
                    order.push(Vec::new());
                }
                order[level].push(root.borrow().val);

                if root.borrow().left.is_some() {
                    queue.push_back((level + 1, root.borrow().left.clone()));
                }
                if root.borrow().right.is_some() {
                    queue.push_back((level + 1, root.borrow().right.clone()));
                }
            }
        }

        order
        // 0ms/2.18MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![vec![1]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<_>>::new());
    }
}
