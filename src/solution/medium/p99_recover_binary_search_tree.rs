use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [99. 恢复二叉搜索树](https://leetcode.cn/problems/recover-binary-search-tree/)
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        type TreeType = Option<Rc<RefCell<TreeNode>>>;

        fn check(swap: &mut (TreeType, TreeType), prev: TreeType, curr: TreeType) {
            if let (Some(prev), Some(curr)) = (prev, curr) {
                if prev.borrow().val > curr.borrow().val {
                    if swap.0.is_none() {
                        swap.0 = Some(prev.clone());
                    }
                    swap.1 = Some(curr.clone());
                }
            }
        }

        if root.is_none() {
            return;
        }

        let mut swap = (None, None);
        let (mut prev, mut curr) = (None, root.clone());
        while let Some(node_c) = curr.clone() {
            if let Some(mut node_l) = node_c.borrow().left.clone() {
                while node_l.borrow().right.is_some() && node_l.borrow().right != curr {
                    let node_t = node_l.borrow().right.clone().unwrap();
                    node_l = node_t;
                }

                if node_l.borrow().right.is_some() {
                    check(&mut swap, prev, curr.clone());
                    node_l.borrow_mut().right = None;
                    prev = curr.clone();
                    curr = node_c.borrow().right.clone();
                } else {
                    node_l.borrow_mut().right = curr.clone();
                    curr = node_c.borrow().left.clone();
                }
            } else {
                check(&mut swap, prev, curr.clone());
                prev = curr.clone();
                curr = node_c.borrow().right.clone();
            }
        }

        if let (Some(s0), Some(s1)) = swap {
            std::mem::swap(&mut s0.borrow_mut().val, &mut s1.borrow_mut().val);
        }
        // 0ms/2.12MB
    }

    // pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     let mut root = root.clone();

    //     let mut stack = Vec::new();
    //     let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
    //     let mut swap = (None, None);
    //     loop {
    //         while let Some(node) = root {
    //             stack.push(node.clone());
    //             root = node.borrow().left.clone()
    //         }

    //         if stack.is_empty() {
    //             break;
    //         }

    //         if let Some(node) = stack.pop() {
    //             if let Some(prev) = prev {
    //                 if node.borrow().val < prev.borrow().val {
    //                     swap.1 = Some(node.clone());
    //                     if swap.0.is_none() {
    //                         swap.0 = Some(prev.clone());
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //             }

    //             prev = Some(node.clone());
    //             root = node.borrow().right.clone();
    //         }
    //     }

    //     if let (Some(s0), Some(s1)) = swap {
    //         std::mem::swap(&mut s0.borrow_mut().val, &mut s1.borrow_mut().val);
    //     }
    //     // 8ms/2.05MB
    // }

    // pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    //     let tree = root.clone();

    //     let swap = {
    //         let (mut list, mut stack) = (Vec::new(), Vec::new());
    //         let mut root = tree.clone();

    //         let mut index = (None, None);
    //         loop {
    //             while let Some(node) = root {
    //                 stack.push(node.clone());
    //                 root = node.borrow().left.clone()
    //             }

    //             if stack.is_empty() {
    //                 break (list[index.0.unwrap()], list[index.1.unwrap()]);
    //             }

    //             if let Some(node) = stack.pop() {
    //                 if let Some(&last) = list.last() {
    //                     if node.borrow().val < last {
    //                         index.1 = Some(list.len());
    //                         if index.0.is_none() {
    //                             index.0 = Some(list.len() - 1);
    //                         } else {
    //                             break (list[index.0.unwrap()], node.borrow().val);
    //                         }
    //                     }
    //                 }
    //                 list.push(node.borrow().val);
    //                 root = node.borrow().right.clone();
    //             }
    //         }
    //     };

    //     {
    //         let (mut list, mut stack) = (Vec::new(), Vec::new());
    //         let mut root = tree.clone();

    //         'outer: loop {
    //             while let Some(node) = root {
    //                 if node.borrow().val == swap.0 {
    //                     node.borrow_mut().val = swap.1;
    //                 } else if node.borrow().val == swap.1 {
    //                     node.borrow_mut().val = swap.0;
    //                 }
    //                 stack.push(node.clone());
    //                 root = node.borrow().left.clone()
    //             }

    //             if stack.is_empty() {
    //                 break;
    //             }

    //             if let Some(node) = stack.pop() {
    //                 list.push(node.borrow().val);
    //                 root = node.borrow().right.clone();
    //             }
    //         }
    //     }
    //     // 0ms/2.18MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::from(
            "[1, 3, N, N, 2]".to_string(),
        ))));
        Solution::recover_tree(&mut root);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 1, N, N, 2]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::from(
            "[3, 1, 4, N, N, 2]".to_string(),
        ))));
        Solution::recover_tree(&mut root);
        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[2, 1, 4, N, N, 3]".to_string(),
            ))))
        );
    }
}
