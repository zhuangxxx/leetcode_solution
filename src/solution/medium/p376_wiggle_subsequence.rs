struct Solution;

impl Solution {
    /// [376. 摆动序列](https://leetcode.cn/problems/wiggle-subsequence/)
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut max = 1;

        let mut prev = 0;
        for diff in nums.windows(2).map(|arr| arr[1] - arr[0]) {
            if (prev <= 0 && diff > 0) || (prev >= 0 && diff < 0) {
                max += 1;
                prev = diff;
            }
        }

        max
        // 0ms/2.07MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
}
