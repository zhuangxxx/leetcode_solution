// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
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

impl From<String> for TreeNode {
    fn from(src: String) -> Self {
        if !src.starts_with('[') || !src.ends_with(']') {
            return TreeNode::new(0);
        }

        let src = src
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|i| {
                if let Ok(i) = i.trim().parse::<i32>() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut pos = 0;

        if src.len() == 0 || src[pos].is_none() {
            return TreeNode::new(0);
        }

        let root = Rc::new(RefCell::new(TreeNode::new(src[pos].unwrap())));
        pos += 1;

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(root) = queue.pop_front() {
                    if pos < src.len() {
                        if let Some(val) = src[pos] {
                            let ptr = Rc::new(RefCell::new(TreeNode::new(val)));
                            root.borrow_mut().left = Some(ptr.clone());
                            queue.push_back(ptr.clone());
                        } else {
                            root.borrow_mut().left = None;
                        }

                        pos += 1;
                    }

                    if pos < src.len() {
                        if let Some(val) = src[pos] {
                            let ptr = Rc::new(RefCell::new(TreeNode::new(val)));
                            root.borrow_mut().right = Some(ptr.clone());
                            queue.push_back(ptr.clone());
                        } else {
                            root.borrow_mut().right = None;
                        }

                        pos += 1;
                    }
                }
            }
        }

        let tree = TreeNode {
            val: root.borrow().val,
            left: root.borrow().left.clone(),
            right: root.borrow().right.clone(),
        };

        tree
    }
}

impl Into<String> for TreeNode {
    fn into(self) -> String {
        let mut res = Vec::new();
        res.push(Some(self.val));

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((self.left, self.right));

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some((left, right)) = queue.pop_front() {
                    if let Some(left) = left {
                        res.push(Some(left.borrow().val));
                        queue.push_back((left.borrow().left.clone(), left.borrow().right.clone()));
                    } else {
                        res.push(None);
                    }

                    if let Some(right) = right {
                        res.push(Some(right.borrow().val));
                        queue
                            .push_back((right.borrow().left.clone(), right.borrow().right.clone()));
                    } else {
                        res.push(None);
                    }
                }
            }
        }

        while let Some(&node) = res.last() {
            if node.is_some() {
                break;
            }
            res.pop();
        }

        String::from("[")
            + res
                .iter()
                .map(|&i| {
                    if let Some(i) = i {
                        i.to_string()
                    } else {
                        String::from("N")
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
                .as_str()
            + "]"
    }
}

impl std::fmt::Display for TreeNode {
    /// TODO: Implement `Display` trait for `TreeNode` struct.
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            TreeNode::from(String::from("[1, 2, N, 3]")),
            TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
                right: None,
            }
        );
    }

    #[test]
    fn test_into() {
        let src: String = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: None,
        }
        .into();
        assert_eq!(src, "[1, 2, N, 3]");
    }

    #[test]
    fn test_fmt() {
        println!("{:?}", TreeNode::from(String::from("[1, 2, N, 3]")));
    }
}
