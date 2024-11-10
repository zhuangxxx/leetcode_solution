struct Solution;

impl Solution {
    /// [3255. 长度为 K 的子数组的能量值 II](https://leetcode.cn/problems/find-the-power-of-k-size-subarrays-ii/)
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums;
        }

        let mut result = vec![-1; nums.len() + 1 - k as usize];

        let mut c = 1;
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] != 1 {
                c = 0;
            }
            c += 1;
            if c >= k {
                result[i + 1 - k as usize] = nums[i];
            }
        }

        result
        // 0ms/3.49MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
            vec![3, 4, -1, -1, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::results_array(vec![2, 2, 2, 2, 2], 4),
            vec![-1, -1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
            vec![-1, 3, -1, 3, -1]
        );
    }
}
