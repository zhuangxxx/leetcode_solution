// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    /// [83. 删除排序链表中的重复元素](https://leetcode.cn/problems/remove-duplicates-from-sorted-list/)
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ptr = head.as_mut();

        while let Some(mut node) = ptr.take() {
            while let Some(next) = node.next.as_mut() {
                if node.val != next.val {
                    break;
                }

                node.next = next.next.take();
            }

            ptr = node.next.as_mut();
        }

        head
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(2)))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::delete_duplicates(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode::new(3)))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            }))
        )
    }
}
