/// [707. 设计链表](https://leetcode.cn/problems/design-linked-list/)
struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
    // 15ms/2.38MB
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            val: -1,
            next: None,
        }
    }

    fn get(&self, index: i32) -> i32 {
        let mut next = &self.next;
        for _ in 0..index {
            if let Some(node) = next {
                next = &node.next;
            }
        }
        if let Some(node) = next {
            node.val
        } else {
            -1
        }
    }

    fn add_at_head(&mut self, val: i32) {
        self.next = Some(Box::new(MyLinkedList {
            val,
            next: self.next.take(),
        }));
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.next.is_none() {
            let mut tail = Self::new();
            tail.val = val;
            self.next = Some(Box::new(tail));
        } else {
            let mut next = &mut self.next;
            while let Some(node) = next {
                if node.next.is_none() {
                    let mut tail = Self::new();
                    tail.val = val;
                    node.next = Some(Box::new(tail));
                    break;
                }
                next = &mut node.next;
            }
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
        } else {
            let mut next = &mut self.next;
            for _ in 1..index {
                if let Some(node) = next {
                    next = &mut node.next;
                }
            }
            if let Some(node) = next {
                let mut new = MyLinkedList::new();
                new.val = val;
                new.next = node.next.take();
                node.next = Some(Box::new(new));
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            if let Some(node) = &mut self.next {
                self.next = node.next.take();
            }
        } else {
            let mut next = &mut self.next;
            for _ in 1..index {
                if let Some(node) = next {
                    next = &mut node.next;
                }
            }
            if let Some(node) = next {
                if let Some(mut new) = node.next.take() {
                    node.next = new.next.take();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_head(1);
        my_linked_list.add_at_tail(3);
        my_linked_list.add_at_index(1, 2);
        assert_eq!(my_linked_list.get(1), 2);
        my_linked_list.delete_at_index(1);
        assert_eq!(my_linked_list.get(1), 3);
    }

    #[test]
    fn fail1() {
        let mut my_linked_list = MyLinkedList::new();
        my_linked_list.add_at_head(7);
        my_linked_list.add_at_head(2);
        my_linked_list.add_at_head(1);
        my_linked_list.add_at_index(3, 0);
        my_linked_list.delete_at_index(2);
        my_linked_list.add_at_head(6);
        my_linked_list.add_at_tail(4);
        assert_eq!(my_linked_list.get(4), 4);
        my_linked_list.add_at_head(4);
        my_linked_list.add_at_index(5, 0);
        my_linked_list.add_at_head(6);
    }
}
