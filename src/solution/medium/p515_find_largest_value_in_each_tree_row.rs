use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [515. 在每个树行中找最大值](https://leetcode.cn/problems/find-largest-value-in-each-tree-row/)
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut largest = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            let mut max = i32::MIN;
            for _ in 0..queue.len() {
                if let Some(Some(node)) = queue.pop_front() {
                    max = std::cmp::max(max, node.borrow().val);
                    if node.borrow().left.is_some() {
                        queue.push_back(node.borrow_mut().left.take());
                    }
                    if node.borrow().right.is_some() {
                        queue.push_back(node.borrow_mut().right.take());
                    }
                }
            }
            largest.push(max)
        }

        largest
        // 0ms/2.93MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 3, 2, 5, 3, N, 9]".to_string()
            ))))),
            vec![1, 3, 9]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest_values(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            ))))),
            vec![1, 3]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::largest_values(None), Vec::new());
    }
}
