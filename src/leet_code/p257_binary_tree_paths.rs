use crate::data_struct::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [257. 二叉树的所有路径](https://leetcode.cn/problems/binary-tree-paths/)
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn find_path(root: Option<Rc<RefCell<TreeNode>>>, path: String) -> Vec<String> {
            let (mut paths, mut path) = (Vec::new(), path);
            if let Some(root) = root {
                if !path.is_empty() {
                    path.push_str("->");
                }
                path.push_str(root.borrow().val.to_string().as_str());

                if root.borrow().left.is_some() || root.borrow().right.is_some() {
                    paths.append(&mut find_path(root.borrow().left.clone(), path.clone()));
                    paths.append(&mut find_path(root.borrow().right.clone(), path.clone()));
                } else {
                    paths.push(path);
                }
            }

            paths
        }

        find_path(root, String::new())
        // 0ms/2MB
    }

    // pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    //     let mut paths = Vec::new();
    //     if let Some(root) = root {
    //         let mut queue = std::collections::VecDeque::new();
    //         queue.push_back((root.clone(), String::new()));

    //         while !queue.is_empty() {
    //             for _ in 0..queue.len() {
    //                 if let Some((root, mut path)) = queue.pop_front() {
    //                     if !path.is_empty() {
    //                         path.push_str("->");
    //                     }
    //                     path.push_str(root.borrow().val.to_string().as_str());

    //                     if root.borrow().left.is_none() && root.borrow().right.is_none() {
    //                         paths.push(path);
    //                         continue;
    //                     }

    //                     if let Some(left) = root.borrow().left.clone() {
    //                         queue.push_back((left, path.clone()));
    //                     }
    //                     if let Some(right) = root.borrow().right.clone() {
    //                         queue.push_back((right, path.clone()));
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     paths
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, N, 5]".to_string()
            ))))),
            vec!["1->2->5".to_string(), "1->3".to_string()] // DFS
                                                            // vec!["1->3".to_string(), "1->2->5".to_string()] // BFS
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![String::from("1")]
        );
    }
}
