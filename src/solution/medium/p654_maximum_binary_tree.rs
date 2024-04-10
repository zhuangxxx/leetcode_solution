use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [654. 最大二叉树](https://leetcode.cn/problems/maximum-binary-tree/)
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut stack, mut nodes) = (Vec::<usize>::new(), vec![None; nums.len()]);
        for i in 0..nums.len() {
            nodes[i] = Some(Rc::new(RefCell::new(TreeNode::new(nums[i]))));
            while let Some(&j) = stack.last() {
                if nums[i] <= nums[j] {
                    if let Some(node) = nodes[j].clone() {
                        node.borrow_mut().right = nodes[i].clone();
                    }
                    break;
                }
                if let Some(node) = nodes[i].clone() {
                    node.borrow_mut().left = nodes[j].clone();
                    stack.pop();
                }
            }
            stack.push(i);
        }

        nodes[stack[0]].take()
        // 5ms/2.14MB
    }

    // pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn part(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    //         let mut max = 0;
    //         for i in 0..nums.len() {
    //             if nums[i] > nums[max] {
    //                 max = i;
    //             }
    //         }
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             left: if max == 0 { None } else { part(&nums[..max]) },
    //             val: nums[max],
    //             right: if max == nums.len() - 1 {
    //                 None
    //             } else {
    //                 part(&nums[max + 1..])
    //             },
    //         })))
    //     }

    //     part(&nums)
    //     // 8ms/2.24MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[6, 3, 5, N, 2, 0, N, N, 1]".to_string()
            ))))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1]),
            Some(Rc::new(RefCell::new(TreeNode::from(
                "[3, N, 2, N, 1]".to_string()
            ))))
        );
    }
}
