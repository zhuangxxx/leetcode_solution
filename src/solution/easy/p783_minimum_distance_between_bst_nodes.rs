use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [783. 二叉搜索树节点最小距离](https://leetcode.cn/problems/minimum-distance-between-bst-nodes/)
    pub fn min_diff_in_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut min, mut prev) = (i32::MAX, -100000);

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
                min = std::cmp::min(min, node.borrow().val - prev);
                if min == 1 {
                    break;
                }

                prev = node.borrow().val;
                root = node.borrow().right.clone();
            }
        }

        min
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 6, 1, 3]".to_string()
            ))))),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 0, 48, N, N, 12, 49]".to_string()
            ))))),
            1
        );
    }
}
