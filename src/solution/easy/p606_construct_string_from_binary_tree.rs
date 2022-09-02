use crate::structure::tree_node::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    /// [606. 根据二叉树创建字符串](https://leetcode.cn/problems/construct-string-from-binary-tree/)
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<char> {
            let mut s = Vec::new();

            if let Some(root) = root {
                s.append(&mut format!("{}", root.borrow().val).chars().collect());

                if root.borrow().left.is_none() && root.borrow().right.is_none() {
                    return s;
                }

                s.push('(');
                s.append(&mut dfs(root.borrow().left.clone()));
                s.push(')');

                if root.borrow().right.is_some() {
                    s.push('(');
                    s.append(&mut dfs(root.borrow().right.clone()));
                    s.push(')');
                }
            }

            s
        }

        dfs(root).iter().collect()
        // 0ms/3.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, 4]".to_string(),
            ))))),
            "1(2(4))(3)".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode::from(
                "[1, 2, 3, N, 4]".to_string(),
            ))))),
            "1(2()(4))(3)".to_string()
        );
    }
}
