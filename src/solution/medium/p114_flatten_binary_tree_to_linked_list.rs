use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [114. 二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/)
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root = root.clone();
        while let Some(node) = root {
            let mut take = node.borrow().left.clone();
            while let Some(temp) = take.clone() {
                if temp.borrow().right.is_none() {
                    break;
                }
                take = temp.borrow().right.clone();
            }
            if let Some(temp) = take.clone() {
                let mut right = node.borrow_mut().right.take();
                temp.borrow_mut().right = right.take();
                right = node.borrow_mut().left.take();
                node.borrow_mut().right = right.take();
            }
            root = node.borrow().right.clone();
        }
        // 0ms/2.24MB
    }

    // pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    //         if let Some(root) = root {
    //             if root.borrow().left.is_some() && root.borrow().right.is_some() {
    //                 let mut right = root.borrow_mut().right.take();
    //                 let mut left = root.borrow_mut().left.take();
    //                 root.borrow_mut().right = left.take();
    //                 if let Some(leaf) = dfs(root.borrow().right.clone()) {
    //                     leaf.borrow_mut().right = right.take();
    //                     dfs(leaf.borrow().right.clone())
    //                 } else {
    //                     None
    //                 }
    //             } else if root.borrow().left.is_some() {
    //                 let mut left = root.borrow_mut().left.take();
    //                 root.borrow_mut().right = left.take();
    //                 dfs(root.borrow().right.clone())
    //             } else if root.borrow().right.is_some() {
    //                 dfs(root.borrow().right.clone())
    //             } else {
    //                 Some(root)
    //             }
    //         } else {
    //             None
    //         }
    //     }

    //     dfs(root.clone());
    //     // 0ms/2.23MB
    // }
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Some(Rc::new(RefCell::new(TreeNode::from(
            "[1, 2, 5, 3, 4, N, 6]".to_string(),
        ))));
        Solution::flatten(&mut result);
        assert_eq!(
            result,
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, N, 3, N, 4, N, 5, N, 6]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        let mut result = None;
        Solution::flatten(&mut result);
        assert_eq!(result, None);
    }

    #[test]
    fn test3() {
        let mut result = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        Solution::flatten(&mut result);
        assert_eq!(result, Some(Rc::new(RefCell::new(TreeNode::new(0)))));
    }

    #[test]
    fn fail1() {
        let mut result = Some(Rc::new(RefCell::new(TreeNode::from(
            "[1, 2, N, 3, 4, 5]".to_string(),
        ))));
        Solution::flatten(&mut result);
        assert_eq!(
            result,
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 2, N, 3, N, 5, N, 4]".to_string()
            ))))
        );
    }
}
