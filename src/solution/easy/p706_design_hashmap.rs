/// [706. 设计哈希映射](https://leetcode.cn/problems/design-hashmap/)
struct MyHashMap {
    nums: Vec<Vec<[i32; 2]>>,
    // 28ms/6.8MB
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            nums: vec![vec![]; 32],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let m = (key & 31) as usize;
        for i in 0..self.nums[m].len() {
            if self.nums[m][i][0] == key {
                self.nums[m][i][1] = value;
                return;
            }
        }
        self.nums[m].push([key, value]);
    }

    fn get(&self, key: i32) -> i32 {
        let m = (key & 31) as usize;
        for i in 0..self.nums[m].len() {
            if self.nums[m][i][0] == key {
                return self.nums[m][i][1];
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let m = (key & 31) as usize;
        for i in 0..self.nums[m].len() {
            if self.nums[m][i][0] == key {
                self.nums[m].remove(i);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_hash_map = MyHashMap::new();
        my_hash_map.put(1, 1);
        my_hash_map.put(2, 2);
        assert_eq!(my_hash_map.get(1), 1);
        assert_eq!(my_hash_map.get(3), -1);
        my_hash_map.put(2, 1);
        assert_eq!(my_hash_map.get(2), 1);
        my_hash_map.remove(2);
        assert_eq!(my_hash_map.get(2), -1);
    }
}
