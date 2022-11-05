struct Solution;

impl Solution {
    /// [746. 使用最小花费爬楼梯](https://leetcode.cn/problems/min-cost-climbing-stairs/)
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let (mut prev, mut curr) = (0, 0);
        for i in 2..=cost.len() {
            let next = std::cmp::min(curr + cost[i - 1], prev + cost[i - 2]);
            prev = curr;
            curr = next;
        }

        curr
        // 0ms/2.2MB
    }

    // pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    //     let mut dp = vec![0; cost.len() + 1];
    //     for i in 2..=cost.len() {
    //         dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
    //     }

    //     dp[cost.len()]
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 1, 2, 2]), 2)
    }
}
