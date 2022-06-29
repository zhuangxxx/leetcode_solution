// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [108. 将有序数组转换为二叉搜索树](https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/)
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn sorted_slice_to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match nums.len() {
                0 => None,
                1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
                _ => Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[nums.len() / 2],
                    left: sorted_slice_to_bst(&nums[0..nums.len() / 2]),
                    right: sorted_slice_to_bst(&nums[nums.len() / 2 + 1..nums.len()]),
                }))),
            }
        }

        sorted_slice_to_bst(&nums[..])
        // 0ms/2.7MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None
                })))
            })))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![1, 3]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None
            })))
        );
    }
}
