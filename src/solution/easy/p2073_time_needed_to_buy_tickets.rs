struct Solution;

impl Solution {
    /// [2073. 买票需要的时间](https://leetcode.cn/problems/time-needed-to-buy-tickets/)
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let (k, c) = (k as usize, tickets[k as usize]);
        let mut sum = 0;
        for (i, t) in tickets.into_iter().enumerate() {
            if t < c {
                sum += t;
            } else if i <= k {
                sum += c;
            } else {
                sum += c - 1;
            }
        }

        sum
        // 2ms/2.03MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::time_required_to_buy(vec![1, 2, 3, 4, 5, 5, 3, 2, 1], 4),
            25
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::time_required_to_buy(vec![1, 2, 3, 5, 5, 5, 3, 2, 1], 4),
            26
        );
    }
}
