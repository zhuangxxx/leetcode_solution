struct Solution;

impl Solution {
    /// [1387. 将整数按权重排序](https://leetcode.cn/problems/sort-integers-by-the-power-value/)
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        for i in lo..=hi {
            let (mut j, mut n) = (i, 0);
            while j > 1 {
                if j & 1 == 0 {
                    j >>= 1;
                } else {
                    j = 3 * j + 1;
                }
                n += 1;
            }
            heap.push(std::cmp::Reverse((n, i)));
        }

        for _ in 1..k {
            heap.pop();
        }

        if let Some(std::cmp::Reverse((_, i))) = heap.pop() {
            i
        } else {
            0
        }
        // 7ms/2.29MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
