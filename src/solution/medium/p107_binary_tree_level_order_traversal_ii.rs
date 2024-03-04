use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [107. 二叉树的层序遍历 II](https://leetcode.cn/problems/binary-tree-level-order-traversal-ii/)
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        let mut result = Vec::new();
        while !queue.is_empty() {
            let mut level = Vec::new();
            for _ in 0..queue.len() {
                if let Some(Some(root)) = queue.pop_front() {
                    level.push(root.borrow().val);
                    if root.borrow().left.is_some() {
                        queue.push_back(root.borrow().left.clone());
                    }
                    if root.borrow().right.is_some() {
                        queue.push_back(root.borrow().right.clone());
                    }
                }
            }
            if !level.is_empty() {
                result.push(level);
            }
        }
        result.reverse();

        result
        // 0ms/2.23MB
    }

    // pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    //     fn dfs(root: Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
    //         if let Some(root) = root {
    //             if result.len() <= level {
    //                 result.push(Vec::new());
    //             }
    //             if root.borrow().left.is_some() {
    //                 dfs(root.borrow().left.clone(), level + 1, result);
    //             }
    //             if root.borrow().right.is_some() {
    //                 dfs(root.borrow().right.clone(), level + 1, result);
    //             }
    //             result[level].push(root.borrow().val);
    //         }
    //     }

    //     let mut result = Vec::new();
    //     dfs(root, 0, &mut result);
    //     result.reverse();

    //     result
    //     // 0ms/2.22MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::level_order_bottom(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))),
            vec![vec![15, 7], vec![9, 20], vec![3]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::level_order_bottom(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![vec![1]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::level_order_bottom(None), Vec::<Vec<_>>::new());
    }
}
