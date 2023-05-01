// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(src: Vec<i32>) -> Self {
        if src.is_empty() {
            return ListNode::new(0);
        }

        let mut list = ListNode::new(src[0]);
        let mut tail = &mut list;

        for &val in src[1..].iter() {
            tail.next = Some(Box::new(ListNode::new(val)));
            tail = tail.next.as_mut().unwrap();
        }

        list
    }
}

impl Into<Vec<i32>> for ListNode {
    fn into(mut self) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(self.val);

        while let Some(next) = self.next {
            res.push(next.val);
            self = *next;
        }

        res
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain: Vec<i32> = self.to_owned().into();
        write!(
            f,
            "{}",
            chain
                .iter()
                .map(|&i| i.to_string())
                .collect::<Vec<_>>()
                .join(" -> ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            ListNode::from(vec![1, 2, 3]),
            ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }
        );
    }

    #[test]
    fn test_into() {
        let src: Vec<i32> = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }
        .into();
        assert_eq!(src, vec![1, 2, 3]);
    }

    #[test]
    fn test_format() {
        assert_eq!(
            format!("{}", ListNode::from(vec![1, 2, 3])),
            String::from("1 -> 2 -> 3")
        );
    }
}
