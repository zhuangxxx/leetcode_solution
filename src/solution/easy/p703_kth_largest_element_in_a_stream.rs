/// [703. 数据流中的第 K 大元素](https://leetcode.cn/problems/kth-largest-element-in-a-stream/)
struct KthLargest {
    nums: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    // 8ms/6.4MB
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth = Self {
            nums: nums.iter().map(|&val| std::cmp::Reverse(val)).collect(),
        };

        kth.nums.push(std::cmp::Reverse(i32::MIN));
        while kth.nums.len() > k as usize {
            kth.nums.pop();
        }

        kth
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(std::cmp::Reverse(val));
        self.nums.pop();

        self.nums.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }
}
