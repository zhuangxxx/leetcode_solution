use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [236. 二叉树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/)
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        if let Some(node) = root.clone() {
            let (l, r) = (
                Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone()),
                Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone()),
            );
            if l.is_some() && r.is_some() {
                root
            } else if l.is_some() {
                l
            } else {
                r
            }
        } else {
            root
        }
        // 0ms/4.49MB
    }

    // pub fn lowest_common_ancestor(
    //     root: Option<Rc<RefCell<TreeNode>>>,
    //     p: Option<Rc<RefCell<TreeNode>>>,
    //     q: Option<Rc<RefCell<TreeNode>>>,
    // ) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn dfs(
    //         root: Option<Rc<RefCell<TreeNode>>>,
    //         p: Option<Rc<RefCell<TreeNode>>>,
    //         q: Option<Rc<RefCell<TreeNode>>>,
    //         result: &mut Option<Rc<RefCell<TreeNode>>>,
    //     ) -> bool {
    //         if let Some(node) = root.clone() {
    //             match (
    //                 dfs(node.borrow().left.clone(), p.clone(), q.clone(), result),
    //                 dfs(node.borrow().right.clone(), p.clone(), q.clone(), result),
    //             ) {
    //                 (true, true) => {
    //                     *result = root;
    //                     true
    //                 }
    //                 (false, false) => p == root || q == root,
    //                 _ => {
    //                     if p == root || q == root {
    //                         *result = root;
    //                     }
    //                     true
    //                 }
    //             }
    //         } else {
    //             false
    //         }
    //     }

    //     let mut result = None;
    //     dfs(root, p, q, &mut result);

    //     result
    //     // 4ms/4.40MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[3, 5, 1, 6, 2, 0, 8, N, N, 7, 4]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 6, 2, N, N, 7, 4]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, 0, 8]".to_string()
                ))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 5, 1, 6, 2, 0, 8, N, N, 7, 4]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[3, 5, 1, 6, 2, 0, 8, N, N, 7, 4]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 6, 2, N, N, 7, 4]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::new(4))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 6, 2, N, N, 7, 4]".to_string()
            ))))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode::from("[1, 2]".to_string())))),
                Some(Rc::new(RefCell::new(TreeNode::from("[1, 2]".to_string())))),
                Some(Rc::new(RefCell::new(TreeNode::new(2))))
            ),
            Some(Rc::new(RefCell::new(TreeNode::from("[1, 2]".to_string()))))
        );
    }
}
