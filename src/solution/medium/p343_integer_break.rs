struct Solution;

impl Solution {
    /// [343. 整数拆分](https://leetcode.cn/problems/integer-break/)
    pub fn integer_break(mut n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            4 => 4,
            _ => {
                let mut m = 1;
                while n > 4 {
                    m *= 3;
                    n -= 3;
                }

                m * n
            }
        }
        // 0ms/2.11MB
    }

    // pub fn integer_break(mut n: i32) -> i32 {
    //     if n < 3 {
    //         return n - 1;
    //     }

    //     let n = n as usize;
    //     let mut dp = vec![0; n + 1];
    //     dp[2] = 1;
    //     for i in 3..=n {
    //         dp[i] = std::cmp::max(
    //             2 * std::cmp::max(i - 2, dp[i - 2]),
    //             3 * std::cmp::max(i - 3, dp[i - 3]),
    //         );
    //     }

    //     dp[n] as i32
    //     // 0ms/2.00MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
