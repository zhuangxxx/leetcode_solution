use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [538. 把二叉搜索树转换为累加树](https://leetcode.cn/problems/convert-bst-to-greater-tree/)
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(node) = root {
                dfs(node.borrow().right.clone(), sum);
                *sum += node.borrow().val;
                node.borrow_mut().val = *sum;
                dfs(node.borrow().left.clone(), sum);
            }
        }

        dfs(root.clone(), &mut 0);

        root
        // 3ms/3.02MB
    }
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[4, 1, 6, 0, 2, 5, 7, N, N, N, 3, N, N, N, 8]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[30, 36, 21, 36, 35, 26, 15, N, N, N, 33, N, N, N, 8]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[0, N, 1]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, N, 1]".to_string()
            ))))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 0, 2]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 3, 2]".to_string()
            ))))
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::convert_bst(Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 2, 4, 1]".to_string()
            ))))),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[7, 9, 4, 10]".to_string()
            ))))
        );
    }
}
