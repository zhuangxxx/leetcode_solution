struct Solution;

impl Solution {
    /// [453. 最小操作次数使数组元素相等](https://leetcode.cn/problems/minimum-moves-to-equal-array-elements/)
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (mut min, mut sum) = (i32::MAX, 0);
        for i in 0..nums.len() {
            if nums[i] < min {
                sum += (min - nums[i]) * i as i32;
                min = nums[i];
            } else {
                sum += nums[i] - min;
            }
        }

        sum
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }
}
