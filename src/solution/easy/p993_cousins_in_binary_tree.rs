use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        if let Some(root) = root {
            let (mut xr, mut yr) = (None, None);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((0, root));
            let mut depth = 0;
            while !queue.is_empty() {
                depth += 1;
                for _ in 0..queue.len() {
                    if let Some((parent, root)) = queue.pop_front() {
                        if root.borrow().val == x {
                            xr = Some((parent, depth));
                        }
                        if root.borrow().val == y {
                            yr = Some((parent, depth));
                        }
                        if xr.is_some() && yr.is_some() {
                            queue.clear();
                            break;
                        }
                        if let Some(left) = root.borrow().left.clone() {
                            queue.push_back((root.borrow().val, left));
                        }
                        if let Some(right) = root.borrow().right.clone() {
                            queue.push_back((root.borrow().val, right));
                        }
                    }
                }
            }

            if let (Some((xp, xd)), Some((yp, yd))) = (xr, yr) {
                xp != yp && xd == yd
            } else {
                false
            }
        } else {
            false
        }
        // 0ms/2.2MB
    }

    // pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    //     if let Some(root) = root {
    //         let (mut xr, mut yr) = (None, None);
    //         let mut stack = Vec::new();
    //         stack.push((0, 0, root));
    //         while let Some((parent, depth, root)) = stack.pop() {
    //             if root.borrow().val == x {
    //                 xr = Some((parent, depth));
    //             }
    //             if root.borrow().val == y {
    //                 yr = Some((parent, depth));
    //             }
    //             if xr.is_some() && yr.is_some() {
    //                 break;
    //             }
    //             if let Some(right) = root.borrow().right.clone() {
    //                 stack.push((root.borrow().val, depth + 1, right));
    //             }
    //             if let Some(left) = root.borrow().left.clone() {
    //                 stack.push((root.borrow().val, depth + 1, left));
    //             }
    //         }

    //         if let (Some((xp, xd)), Some((yp, yd))) = (xr, yr) {
    //             xp != yp && xd == yd
    //         } else {
    //             false
    //         }
    //     } else {
    //         false
    //     }
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4]".to_string()
            )))),
            4,
            3
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, N, 4, N, 5]".to_string()
            )))),
            5,
            4
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_cousins(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, N, 4]".to_string()
            )))),
            2,
            3
        ));
    }
}
