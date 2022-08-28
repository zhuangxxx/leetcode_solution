struct Solution;

impl Solution {
    /// [561. 数组拆分](https://leetcode.cn/problems/array-partition/)
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.iter().step_by(2).sum()
        // 12ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
