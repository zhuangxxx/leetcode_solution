use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [450. 删除二叉搜索树中的节点](https://leetcode.cn/problems/delete-node-in-a-bst/)
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let val = node.borrow().val;
            match val.cmp(&key) {
                std::cmp::Ordering::Greater => {
                    let mut new = Self::delete_node(node.borrow_mut().left.take(), key);
                    node.borrow_mut().left = new.take();
                }
                std::cmp::Ordering::Less => {
                    let mut new = Self::delete_node(node.borrow_mut().right.take(), key);
                    node.borrow_mut().right = new.take();
                }
                _ => {
                    return if node.borrow().right.is_some() {
                        let mut right = node.borrow().right.clone();
                        while let Some(r) = right {
                            if r.borrow().left.is_none() {
                                r.borrow_mut().left = node.borrow_mut().left.take();
                                break;
                            }
                            right = r.borrow().left.clone();
                        }
                        node.borrow_mut().right.take()
                    } else {
                        node.borrow_mut().left.take()
                    };
                }
            }
        }

        root
        // 3ms/3.06MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::delete_node(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 3, 6, 2, 4, N, 7]".to_string()
                )))),
                3
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 4, 6, 2, N, N, 7]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::delete_node(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 3, 6, 2, 4, N, 7]".to_string()
                )))),
                0
            ),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 3, 6, 2, 4, N, 7]".to_string()
            ))))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::delete_node(None, 0), None);
    }
}
