struct Solution;

impl Solution {
    /// [1049. 最后一块石头的重量 II](https://leetcode.cn/problems/last-stone-weight-ii/)
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let full = stones.iter().sum::<i32>();
        let half = full as usize >> 1;

        let mut dp = vec![0; half + 1];
        for j in stones[0] as usize..=half {
            dp[j] = stones[0];
        }
        for i in 1..stones.len() {
            for j in (1..=half).rev() {
                if j as i32 >= stones[i] {
                    dp[j] = std::cmp::max(dp[j], dp[j - stones[i] as usize] + stones[i]);
                }
            }
        }

        full - (dp[half] << 1)
        // 2ms/2.09MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
