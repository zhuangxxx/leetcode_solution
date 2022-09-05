use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [637. 二叉树的层平均值](https://leetcode.cn/problems/average-of-levels-in-binary-tree/)
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut avg = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while !queue.is_empty() {
            let len = queue.len();
            let mut sum = 0.0;
            for _ in 0..len {
                if let Some(root) = queue.pop_front() {
                    if let Some(root) = root {
                        sum += root.borrow().val as f64;
                        if root.borrow().left.is_some() {
                            queue.push_back(root.borrow().left.clone());
                        }
                        if root.borrow().right.is_some() {
                            queue.push_back(root.borrow().right.clone());
                        }
                    }
                }
            }
            avg.push(sum / len as f64);
        }

        avg
        // 0ms/2.9MB
    }

    // pub fn average_of_levels(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    //     let (mut level, mut count) = (0, Vec::new());

    //     let mut stack = Vec::new();
    //     loop {
    //         while let Some(node) = root {
    //             if level == count.len() {
    //                 count.push((0.0, 0));
    //             }
    //             count[level] = (
    //                 count[level].0 + node.borrow().val as f64,
    //                 count[level].1 + 1,
    //             );

    //             level += 1;
    //             root = node.borrow().left.clone();

    //             stack.push((level, node));
    //         }

    //         if stack.is_empty() {
    //             break;
    //         }

    //         if let Some((l, node)) = stack.pop() {
    //             level = l;
    //             root = node.borrow().right.clone();
    //         }
    //     }

    //     count
    //         .iter()
    //         .map(|&(total, count)| total / count as f64)
    //         .collect()
    //     // 0ms/3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let avg = Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode::from(
            "[3, 9, 20, N, N, 15, 7]".to_string(),
        )))));

        assert!((avg[0] - 3.0).abs() < f64::EPSILON);
        assert!((avg[1] - 14.5).abs() < f64::EPSILON);
        assert!((avg[2] - 11.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test2() {
        let avg = Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode::from(
            "[3, 9, 20, 15, 7]".to_string(),
        )))));

        assert!((avg[0] - 3.0).abs() < f64::EPSILON);
        assert!((avg[1] - 14.5).abs() < f64::EPSILON);
        assert!((avg[2] - 11.0).abs() < f64::EPSILON);
    }

    #[test]
    fn fail1() {
        let avg = Solution::average_of_levels(Some(Rc::new(RefCell::new(TreeNode::from(
            "[2147483647, 2147483647, 2147483647]".to_string(),
        )))));

        assert!((avg[0] - 2147483647.0).abs() < f64::EPSILON);
        assert!((avg[1] - 2147483647.0).abs() < f64::EPSILON);
    }
}
