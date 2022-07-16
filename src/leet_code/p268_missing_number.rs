struct Solution;

impl Solution {
    /// [268. 丢失的数字](https://leetcode.cn/problems/missing-number/)
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let tag = ((nums.len() + 1) * nums.len() / 2) as i32;

        let mut sum = 0;
        for num in nums {
            sum += num;
        }

        tag - sum
        // 0ms/2.2MB
    }

    // pub fn missing_number(nums: Vec<i32>) -> i32 {
    //     let mut xor = 0;

    //     for i in 0..nums.len() {
    //         xor ^= nums[i];
    //     }

    //     for i in 0..=nums.len() {
    //         xor ^= i as i32;
    //     }

    //     xor
    //     // 4ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::missing_number(vec![0]), 1);
    }
}
