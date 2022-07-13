use crate::data_struct::list_node::ListNode;

/// TODO: Implement `Recursive` and `Fast & Slow Pointer` Solution
struct Solution;

impl Solution {
    /// [234. 回文链表](https://leetcode.cn/problems/palindrome-linked-list/)
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if let Some(head) = head {
            let (mut tail, mut values) = (head.as_ref(), vec![head.val]);
            while let Some(node) = tail.next.as_ref() {
                values.push(node.val);
                tail = node.as_ref();
            }

            tail = head.as_ref();
            while let Some(node) = tail.next.as_ref() {
                if values.is_empty() || values.pop().unwrap() != tail.val {
                    return false;
                }

                tail = node.as_ref();
            }
        }

        true
        // 48ms/9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(Some(Box::new(ListNode::from(
            vec![1, 2, 2, 1]
        )))));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome(Some(Box::new(ListNode::from(
            vec![1, 2]
        )))));
    }
}
