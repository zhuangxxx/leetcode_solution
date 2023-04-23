use crate::structure::list_node::ListNode;

struct Solution;

impl Solution {
    /// [86. 分隔链表](https://leetcode.cn/problems/partition-list/)
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let (mut lt, mut ge) = (ListNode::new(0), ListNode::new(0));
        let (mut ltp, mut gep) = (&mut lt, &mut ge);
        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                ltp.next.replace(node);
                ltp = ltp.next.as_mut().unwrap();
            } else {
                gep.next.replace(node);
                gep = gep.next.as_mut().unwrap();
            }
        }
        ltp.next = ge.next.take();

        lt.next
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition(Some(Box::new(ListNode::from(vec![1, 4, 3, 2, 5, 2]))), 3),
            Some(Box::new(ListNode::from(vec![1, 2, 2, 4, 3, 5])))
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::partition(Some(Box::new(ListNode::from(vec![2, 1]))), 2),
            Some(Box::new(ListNode::from(vec![1, 2])))
        );
    }
}
