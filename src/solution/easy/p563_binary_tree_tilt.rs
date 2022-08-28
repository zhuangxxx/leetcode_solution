use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [563. 二叉树的坡度](https://leetcode.cn/problems/binary-tree-tilt/)
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, tilt: &mut i32) -> i32 {
            if let Some(root) = root {
                let (l, r) = (
                    dfs(root.borrow().left.clone(), tilt),
                    dfs(root.borrow().right.clone(), tilt),
                );
                *tilt += (l - r).abs();

                l + r + root.borrow().val
            } else {
                0
            }
        }

        let mut tilt = 0;
        dfs(root, &mut tilt);

        tilt
        // 4ms/2.6MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3]".to_string()
            ))))),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 2, 9, 3, 5, N, 7]".to_string()
            ))))),
            15
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode::from(
                "[21, 7, 14, 1, 1, 2, 2, 3, 3]".to_string()
            ))))),
            9
        );
    }
}
