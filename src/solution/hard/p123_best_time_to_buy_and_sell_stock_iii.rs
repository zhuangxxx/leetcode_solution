struct Solution;

impl Solution {
    /// [123. 买卖股票的最佳时机 III](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy1, mut sell1, mut buy2, mut sell2) = (-prices[0], 0, -prices[0], 0);
        for price in prices.into_iter().skip(1) {
            buy1 = std::cmp::max(buy1, -price);
            sell1 = std::cmp::max(sell1, buy1 + price);
            buy2 = std::cmp::max(buy2, sell1 - price);
            sell2 = std::cmp::max(sell2, buy2 + price);
        }

        sell2
        // 11ms/3.15MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 7);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::max_profit(vec![2, 1, 2, 1, 0, 0, 1]), 2);
    }
}
