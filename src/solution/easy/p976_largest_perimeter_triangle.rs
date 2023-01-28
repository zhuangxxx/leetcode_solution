struct Solution;

impl Solution {
    /// [976. 三角形的最大周长](https://leetcode.cn/problems/largest-perimeter-triangle/)
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = nums.len() - 1;
        while i >= 2 {
            if nums[i - 2] + nums[i - 1] > nums[i] {
                return nums[i - 2] + nums[i - 1] + nums[i];
            }
            i -= 1;
        }

        0
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1, 10]), 0);
    }
}
