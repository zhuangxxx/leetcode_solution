struct Solution;

impl Solution {
    /// [128. 最长连续序列](https://leetcode.cn/problems/longest-consecutive-sequence/)
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut longest = 0;

        let mut map = std::collections::HashMap::new();
        for n in nums {
            if !map.contains_key(&n) {
                let (&left, &right) = (
                    map.get(&(n - 1)).unwrap_or(&0),
                    map.get(&(n + 1)).unwrap_or(&0),
                );
                let len = left + right + 1;
                if len > longest {
                    longest = len;
                }
                map.insert(n, len);
                map.insert(n - left, len);
                map.insert(n + right, len);
            }
        }

        longest
        // 29ms/4.71MB
    }

    // pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    //     if nums.is_empty() {
    //         return 0;
    //     }

    //     let mut longest = 1;

    //     let mut heap = std::collections::BinaryHeap::from(nums);
    //     let (mut prev, mut len) = (heap.pop().unwrap(), 1);
    //     while let Some(n) = heap.pop() {
    //         match prev - n {
    //             0 => {}
    //             1 => len += 1,
    //             _ => {
    //                 longest = std::cmp::max(longest, len);
    //                 len = 1;
    //             }
    //         }
    //         prev = n;
    //     }
    //     longest = std::cmp::max(longest, len);

    //     longest
    //     // 17ms/3.39MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::longest_consecutive(Vec::new()), 0);
    }
}
