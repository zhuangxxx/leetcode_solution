use core::num;

struct Solution;

impl Solution {
    /// [961. 在长度 2N 的数组中找出重复 N 次的元素](https://leetcode.cn/problems/n-repeated-element-in-size-2n-array/)
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        loop {
            let (i, j) = (
                (std::time::Instant::now().elapsed().as_nanos() % (nums.len() as u128)) as usize,
                (std::time::Instant::now().elapsed().as_nanos() % (nums.len() as u128)) as usize,
            );

            if i != j && nums[i] == nums[j] {
                return nums[i];
            }
        }
        // 12ms/2.2MB
    }

    // pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    //     for gap in 1..=3 {
    //         for i in 0..nums.len() - gap {
    //             if nums[i] == nums[i + gap] {
    //                 return nums[i];
    //             }
    //         }
    //     }

    //     0
    //     // 0ms/2.1MB
    // }

    // pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    //     let mut set = std::collections::HashSet::new();
    //     for num in nums {
    //         if !set.insert(num) {
    //             return num;
    //         }
    //     }

    //     0
    //     // 0ms/2.3MB
    // }

    // pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
    //     nums.sort_unstable();

    //     if nums[nums.len() / 2] != nums[nums.len() / 2 - 1] && nums[0] == nums[nums.len() / 2 - 1] {
    //         nums[0]
    //     } else {
    //         nums[nums.len() / 2]
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::repeated_n_times(vec![9, 5, 6, 9]), 9);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::repeated_n_times(vec![2, 6, 2, 1]), 2);
    }
}
