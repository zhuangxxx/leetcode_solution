struct Solution;

impl Solution {
    /// [485. 最大连续 1 的个数](https://leetcode.cn/problems/max-consecutive-ones/)
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max, mut count) = (0, 0);

        for n in nums {
            if n == 0 {
                max = std::cmp::max(max, count);
                count = 0;
            } else {
                count += 1;
            }
        }

        std::cmp::max(max, count)
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
