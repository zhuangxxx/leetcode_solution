struct Solution;

impl Solution {
    /// [674. 最长连续递增序列](https://leetcode.cn/problems/longest-continuous-increasing-subsequence/)
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut len = 1;

        let mut slow = 0;
        for fast in 1..=nums.len() {
            if fast == nums.len() || nums[fast - 1] >= nums[fast] {
                len = std::cmp::max(len, fast - slow);
                slow = fast;
            }
        }

        len as i32
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
