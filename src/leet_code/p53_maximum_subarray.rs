pub struct Solution;

impl Solution {
    /// [53. 最大子数组和](https://leetcode.cn/problems/maximum-subarray/)
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut prev, mut max) = (0, nums[0]);

        for num in nums {
            prev = std::cmp::max(prev + num, num);
            max = std::cmp::max(prev, max);
        }

        max
        // 12ms/3.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
