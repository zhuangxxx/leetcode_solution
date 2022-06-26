// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    /// [21. 合并两个有序链表](https://leetcode-cn.com/problems/merge-two-sorted-lists/)
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut head = ListNode::new(0);
        let mut tail = &mut head;

        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            if node1.val < node2.val {
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        tail.next = list1.or(list2);

        head.next
        // 0ms/2.1MB
    }

    // pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>)  -> Option<Box<ListNode>> {
    //     match (list1, list2) {
    //         (Some(mut node1), Some(mut node2)) => {
    //             if node1.val < node2.val {
    //                 node1.next = Solution::merge_two_lists(node1.next, Some(node2));
    //                 Some(node1)
    //             } else {
    //                 node2.next = Solution::merge_two_lists(Some(node1), node2.next);
    //                 Some(node2)
    //             }
    //         }
    //         (Some(node1), None) => Some(node1),
    //         (None, Some(node2)) => Some(node2),
    //         (None, None) => None,
    //     }
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(4))),
                            })),
                        })),
                    })),
                })),
            }))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::merge_two_lists(None, Some(Box::new(ListNode::new(1)))),
            Some(Box::new(ListNode::new(1)))
        );
    }
}
