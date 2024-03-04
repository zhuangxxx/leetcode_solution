use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [113. 路径总和 II](https://leetcode.cn/problems/path-sum-ii/)
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            sum: i32,
            path: &mut Vec<i32>,
        ) -> Vec<Vec<i32>> {
            if let Some(root) = root {
                path.push(root.borrow().val);
                let result = if root.borrow().left.is_some() || root.borrow().right.is_some() {
                    let mut result = Vec::new();
                    result.append(&mut dfs(
                        root.borrow().left.clone(),
                        sum - root.borrow().val,
                        path,
                    ));
                    result.append(&mut dfs(
                        root.borrow().right.clone(),
                        sum - root.borrow().val,
                        path,
                    ));

                    result
                } else if sum == root.borrow().val {
                    vec![path.clone()]
                } else {
                    Vec::new()
                };
                path.pop();

                result
            } else {
                Vec::new()
            }
        }

        dfs(root, target_sum, &mut Vec::new())
        // 2ms/2.69MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::path_sum(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[5, 4, 8, 11, N, 13, 4, 7, 2, N, N, 5, 1]".to_string()
                )))),
                22
            ),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::path_sum(
                Some(Rc::new(RefCell::new(TreeNode::from(
                    "[1, 2, 3]".to_string()
                )))),
                5
            ),
            Vec::<Vec<_>>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::path_sum(
                Some(Rc::new(RefCell::new(TreeNode::from("[1, 2]".to_string())))),
                0
            ),
            Vec::<Vec<_>>::new()
        );
    }
}
