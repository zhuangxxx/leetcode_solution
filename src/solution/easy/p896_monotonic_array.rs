use core::num;

struct Solution;

impl Solution {
    /// [896. 单调数列](https://leetcode.cn/problems/monotonic-array/)
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let (mut inc, mut dec) = (true, true);
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                inc = false;
            } else if nums[i] < nums[i + 1] {
                dec = false;
            }
        }

        inc || dec
        // 8ms/3.2MB
    }

    // pub fn is_monotonic(nums: Vec<i32>) -> bool {
    //     if nums.len() <= 2 {
    //         return true;
    //     }

    //     let mut ord = nums[1].cmp(&nums[0]);
    //     for i in 2..nums.len() {
    //         if nums[i] != nums[i - 1] {
    //             if ord.is_eq() {
    //                 ord = nums[i].cmp(&nums[i - 1]);
    //             } else if ord != nums[i].cmp(&nums[i - 1]) {
    //                 return false;
    //             }
    //         }
    //     }

    //     true
    //     // 8ms/3.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }

    #[test]
    fn trap1() {
        assert!(Solution::is_monotonic(vec![1, 1, 2, 2]));
    }
}
