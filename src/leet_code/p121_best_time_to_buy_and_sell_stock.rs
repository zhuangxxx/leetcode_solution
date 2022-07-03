struct Solution;

impl Solution {
    /// [121. 买卖股票的最佳时机](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min_price, mut max_profit) = (prices[0], 0);
        for i in 1..prices.len() {
            if max_profit < prices[i] - min_price {
                max_profit = prices[i] - min_price;
            }

            if min_price > prices[i] {
                min_price = prices[i];
            }
        }

        max_profit
        // 12ms/2.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::max_profit(vec![5, 6, 7, 8, 9, 1, 2, 3]), 4);
    }
}
