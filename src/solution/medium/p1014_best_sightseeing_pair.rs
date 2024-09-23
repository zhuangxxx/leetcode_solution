struct Solution;

impl Solution {
    /// [1014. 最佳观光组合](https://leetcode.cn/problems/best-sightseeing-pair/)
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut prev = values[0];
        for i in 1..values.len() {
            max = std::cmp::max(max, prev + values[i] - i as i32);
            prev = std::cmp::max(prev, values[i] + i as i32);
        }

        max
        // 0ms/2.34MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
    }
}
