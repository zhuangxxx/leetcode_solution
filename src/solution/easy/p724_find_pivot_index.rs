struct Solution;

impl Solution {
    /// [724. 寻找数组的中心下标](https://leetcode.cn/problems/find-pivot-index/)
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();

        let mut sum = 0;
        for i in 0..nums.len() {
            if 2 * sum + nums[i] == total {
                return i as i32;
            }
            sum += nums[i];
        }

        -1
        // 4ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, -1, -1, -1, -1]), -1);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, -1, -1, -1, 0]), 2);
    }
}
