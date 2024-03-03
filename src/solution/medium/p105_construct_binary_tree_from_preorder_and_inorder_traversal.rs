use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [105. 从前序与中序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        let (mut stack, mut index) = (vec![root.clone()], 0);
        for val in preorder.into_iter().skip(1) {
            if let Some(Some(node)) = stack.last() {
                let mut node = node.clone();
                if node.borrow().val != inorder[index] {
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    stack.push(node.borrow().left.clone());
                } else {
                    while let Some(Some(last)) = stack.last() {
                        if last.borrow().val != inorder[index] {
                            break;
                        }
                        node = last.clone();
                        stack.pop();
                        index += 1;
                    }
                    node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    stack.push(node.borrow().right.clone());
                }
            }
        }

        root
        // 0ms/2.30MB
    }

    // pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn dfs(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    //         if preorder.is_empty() {
    //             return None;
    //         }

    //         let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    //         if let Some(mid) = inorder.iter().position(|&val| val == preorder[0]) {
    //             if mid > 0 {
    //                 root.borrow_mut().left = dfs(&preorder[1..mid + 1], &inorder[0..mid]);
    //             }
    //             if mid < inorder.len() - 1 {
    //                 root.borrow_mut().right = dfs(&preorder[mid + 1..], &inorder[mid + 1..]);
    //             }
    //         }

    //         Some(root)
    //     }

    //     dfs(&preorder, &inorder)
    //     // 3ms/2.73MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, 9, 20, N, N, 15, 7]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            Some(Rc::new(RefCell::new(TreeNode::new(-1))))
        );
    }
}
