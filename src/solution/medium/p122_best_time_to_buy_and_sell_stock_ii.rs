struct Solution;

impl Solution {
    /// [122. 买卖股票的最佳时机 II](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        let mut hold = -1;
        for window in prices.windows(2) {
            if hold >= 0 {
                if window[0] > window[1] {
                    profit += window[0] - hold;
                    hold = -1;
                }
            } else if window[0] < window[1] {
                hold = window[0];
            }
        }
        if hold >= 0 {
            if let Some(&last) = prices.last() {
                profit += last - hold;
            }
        }

        profit
        // 0ms/2.15MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
