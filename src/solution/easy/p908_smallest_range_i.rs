struct Solution;

impl Solution {
    /// [908. 最小差值 I](https://leetcode.cn/problems/smallest-range-i/)
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (mut x, mut s) = (nums[0], nums[0]);
        for n in nums {
            if n > x {
                x = n
            }
            if n < s {
                s = n
            }
        }

        if x - s > 2 * k {
            x - s - 2 * k
        } else {
            0
        }
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
