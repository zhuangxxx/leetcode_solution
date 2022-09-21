/// [705. 设计哈希集合](https://leetcode.cn/problems/design-hashset/)
struct MyHashSet {
    nums: Vec<Vec<i32>>,
    // 20ms/6.4MB
}

impl MyHashSet {
    fn new() -> Self {
        Self {
            nums: vec![vec![]; 32],
        }
    }

    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            let m = (key & 31) as usize;
            self.nums[m].push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let m = (key & 31) as usize;
        for i in 0..self.nums[m].len() {
            if self.nums[m][i] == key {
                self.nums[m].remove(i);
                break;
            }
        }
    }

    fn contains(&self, key: i32) -> bool {
        let m = (key & 31) as usize;
        for i in 0..self.nums[m].len() {
            if self.nums[m][i] == key {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_hash_set = MyHashSet::new();
        my_hash_set.add(1);
        my_hash_set.add(2);
        assert!(my_hash_set.contains(1));
        assert!(!my_hash_set.contains(3));
        my_hash_set.add(2);
        assert!(my_hash_set.contains(2));
        my_hash_set.remove(2);
        assert!(!my_hash_set.contains(2));
    }
}
