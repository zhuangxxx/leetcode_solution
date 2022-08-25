use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [501. 二叉搜索树中的众数](https://leetcode.cn/problems/find-mode-in-binary-search-tree/)
    pub fn find_mode(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_vec = Vec::new();

        let (mut max, mut curr, mut count) = (0, 0, 0);
        let mut stack = Vec::new();
        loop {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone()
            }

            if stack.is_empty() {
                break;
            }

            if let Some(node) = stack.pop() {
                let val = node.borrow().val;
                if val == curr {
                    count += 1;
                } else {
                    curr = val;
                    count = 1;
                }
                if count > max {
                    max = count;
                    max_vec = vec![val];
                } else if count == max {
                    max_vec.push(val);
                }

                root = node.borrow().right.clone();
            }
        }

        max_vec
        // 4ms/3MB
    }

    // pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     fn dfs(
    //         root: Option<Rc<RefCell<TreeNode>>>,
    //         max_vec: &mut Vec<i32>,
    //         max: &mut i32,
    //         curr: &mut i32,
    //         count: &mut i32,
    //     ) {
    //         if let Some(root) = root {
    //             dfs(root.borrow().left.clone(), max_vec, max, curr, count);

    //             let val = root.borrow().val;
    //             if val == *curr {
    //                 *count += 1
    //             } else {
    //                 *curr = val;
    //                 *count = 1;
    //             }
    //             if count > max {
    //                 *max = *count;
    //                 *max_vec = vec![val];
    //             } else if count == max {
    //                 max_vec.push(val);
    //             }

    //             dfs(root.borrow().right.clone(), max_vec, max, curr, count);
    //         }
    //     }

    //     let mut max_vec = Vec::new();
    //     let (mut max, mut curr, mut count) = (0, 0, 0);
    //     dfs(root, &mut max_vec, &mut max, &mut curr, &mut count);

    //     max_vec
    //     // 4ms/3.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, 2]".to_string()
            ))))),
            vec![2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
            vec![0]
        );
    }
}
