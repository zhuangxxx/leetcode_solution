use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    /// [95. 不同的二叉搜索树 II](https://leetcode.cn/problems/unique-binary-search-trees-ii/)
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn clone(root: Option<Rc<RefCell<TreeNode>>>, n: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root) = root {
                Some(Rc::new(RefCell::new(TreeNode {
                    val: root.borrow().val + n,
                    left: clone(root.borrow().left.clone(), n),
                    right: clone(root.borrow().right.clone(), n),
                })))
            } else {
                root
            }
        }

        let mut dp = vec![Vec::new(); n as usize + 1];
        dp[0] = vec![None];
        dp[1] = vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        for i in 2..=n as usize {
            for j in 1..=i {
                for t in dp[i - j].clone() {
                    let right = clone(t, j as i32);
                    for left in dp[j - 1].clone() {
                        dp[i].push(Some(Rc::new(RefCell::new(TreeNode {
                            val: j as i32,
                            left,
                            right: right.clone(),
                        }))));
                    }
                }
            }
        }

        dp[n as usize].clone()
        // 0ms/2.53MB
    }

    // pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    //     fn gen(b: i32, e: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    //         if b > e {
    //             return vec![None];
    //         }

    //         let mut trees = Vec::new();
    //         for n in b..=e {
    //             let left = gen(b, n - 1);
    //             let right = gen(n + 1, e);
    //             for l in left {
    //                 for r in right.clone() {
    //                     trees.push(Some(Rc::new(RefCell::new(TreeNode {
    //                         val: n,
    //                         left: l.clone(),
    //                         right: r,
    //                     }))));
    //                 }
    //             }
    //         }

    //         trees
    //     }

    //     if n == 0 {
    //         Vec::new()
    //     } else {
    //         gen(1, n)
    //     }
    //     // 0ms/2.51MB
    // }

    // pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    //     fn dfs(root: Option<Rc<RefCell<TreeNode>>>, n: i32) {
    //         if let Some(root) = root {
    //             if n < root.borrow().val {
    //                 if root.borrow().left.is_none() {
    //                     root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(n))));
    //                 } else {
    //                     dfs(root.borrow().left.clone(), n);
    //                 }
    //             } else if root.borrow().right.is_none() {
    //                 root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(n))));
    //             } else {
    //                 dfs(root.borrow().right.clone(), n);
    //             }
    //         }
    //     }

    //     fn is_same(l: &Option<Rc<RefCell<TreeNode>>>, r: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    //         match (l, r) {
    //             (None, None) => true,
    //             (Some(_), None) | (None, Some(_)) => false,
    //             (Some(l), Some(r)) => {
    //                 l.borrow().val == r.borrow().val
    //                     && is_same(&l.borrow().left.clone(), &r.borrow().left.clone())
    //                     && is_same(&l.borrow().right.clone(), &r.borrow().right.clone())
    //             }
    //         }
    //     }

    //     fn generate(nums: Vec<i32>, used: &mut [bool]) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    //         if used.iter().skip(1).all(|&b| b) {
    //             let mut root = Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    //             for n in nums.into_iter().skip(1) {
    //                 dfs(root.clone(), n);
    //             }

    //             return vec![root];
    //         }

    //         let mut trees = Vec::new();
    //         for n in 1..used.len() as i32 {
    //             if used[n as usize] {
    //                 continue;
    //             }
    //             used[n as usize] = true;

    //             let mut nums = nums.clone();
    //             nums.push(n);
    //             unsafe {
    //                 let mut tree = generate(nums, used);
    //                 if !trees
    //                     .iter()
    //                     .any(|temp| is_same(temp, tree.get_unchecked(0)))
    //                 {
    //                     trees.append(&mut tree);
    //                 }
    //             }

    //             used[n as usize] = false;
    //         }

    //         trees
    //     }

    //     generate(Vec::new(), &mut vec![false; n as usize + 1])
    //     // 36ms/2.68MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, N, 2, N, 3]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, N, 3, 2]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[2, 1, 3]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[3, 1, N, N, 2]".to_string()
                )))),
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[3, 2, N, 1]".to_string()
                ))))
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::generate_trees(1),
            vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))]
        );
    }
}
