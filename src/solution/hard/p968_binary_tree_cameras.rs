use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [968. 监控二叉树](https://leetcode.cn/problems/binary-tree-cameras/)
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        #[derive(Eq, PartialEq)]
        enum State {
            Uncovered,
            Camera,
            Covered,
        }
        fn traversal(root: Option<Rc<RefCell<TreeNode>>>, num: &mut i32) -> State {
            if let Some(root) = root {
                match (
                    traversal(root.borrow().left.clone(), num),
                    traversal(root.borrow().right.clone(), num),
                ) {
                    (State::Uncovered, _) | (_, State::Uncovered) => {
                        *num += 1;
                        State::Camera
                    }
                    (State::Camera, _) | (_, State::Camera) => State::Covered,
                    (_, _) => State::Uncovered,
                }
            } else {
                State::Covered
            }
        }

        let mut num = 0;
        if traversal(root, &mut num) == State::Uncovered {
            num + 1
        } else {
            num
        }
        // 2ms/2.14MB
    }

    // pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: i16) -> (i16, i32) {
    //         if let Some(root) = root {
    //             let (ld, ln) = dfs(root.borrow().left.clone(), depth + 1);
    //             let (rd, rn) = dfs(root.borrow().right.clone(), depth + 1);
    //             if depth == ld || depth == rd {
    //                 (depth - 3, ln + rn + 1)
    //             } else {
    //                 (std::cmp::min(ld, rd), ln + rn)
    //             }
    //         } else {
    //             (depth - 2, 0)
    //         }
    //     }

    //     let (depth, num) = dfs(root, 0);
    //     if depth == -1 {
    //         num + 1
    //     } else {
    //         num
    //     }
    //     // 2ms/2.09MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, 0, N, 0, 0]".to_string()
            ))))),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, 0, N, 0, N, 0, N, N, 0]".to_string()
            ))))),
            2
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, 0]".to_string()
            ))))),
            1
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::min_camera_cover(None), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
            1
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, 0, N, N, 0, 0, N, N, 0, 0]".to_string()
            ))))),
            2
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, N, 0, N, 0, N, 0]".to_string()
            ))))),
            2
        );
    }

    #[test]
    fn fail4() {
        assert_eq!(
            Solution::min_camera_cover(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, 0, 0, N, 0, 0, N, N, 0]".to_string()
            ))))),
            2
        );
    }
}
