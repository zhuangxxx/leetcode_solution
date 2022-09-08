use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [653. 两数之和 IV - 输入二叉搜索树](https://leetcode.cn/problems/two-sum-iv-input-is-a-bst/)
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn bfs(root: Option<Rc<RefCell<TreeNode>>>, need: i32, curr: i32) -> bool {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    if let Some(Some(root)) = queue.pop_front() {
                        match need.cmp(&root.borrow().val) {
                            std::cmp::Ordering::Less => queue.push_back(root.borrow().left.clone()),
                            std::cmp::Ordering::Greater => {
                                queue.push_back(root.borrow().right.clone())
                            }
                            std::cmp::Ordering::Equal => {
                                if root.borrow().val != curr {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }

            false
        }

        let (mut stack, mut node) = (Vec::new(), root.clone());
        loop {
            while let Some(n) = node {
                if bfs(root.clone(), k - n.borrow().val, n.borrow().val) {
                    return true;
                }

                stack.push(n.clone());
                node = n.borrow().left.clone();
            }

            if stack.is_empty() {
                break;
            }

            if let Some(n) = stack.pop() {
                node = n.borrow().right.clone();
            }
        }

        false
        // 8ms/3.2MB
    }

    // pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    //     let mut set = std::collections::HashSet::new();

    //     let mut queue = std::collections::VecDeque::new();
    //     queue.push_back(root);
    //     while !queue.is_empty() {
    //         for _ in 0..queue.len() {
    //             if let Some(Some(node)) = queue.pop_front() {
    //                 if set.contains(&(k - node.borrow().val)) {
    //                     return true;
    //                 }
    //                 set.insert(node.borrow().val);
    //                 queue.push_back(node.borrow().left.clone());
    //                 queue.push_back(node.borrow().right.clone());
    //             }
    //         }
    //     }

    //     false
    //     // 8ms/3.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 3, 6, 2, 4, N, 7]".to_string()
            )))),
            9
        ))
    }

    #[test]
    fn test2() {
        assert!(!Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[5, 3, 6, 2, 4, N, 7]".to_string()
            )))),
            28
        ))
    }

    #[test]
    fn fail1() {
        assert!(!Solution::find_target(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            2
        ));
    }
}
