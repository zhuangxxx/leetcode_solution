struct Solution;

impl Solution {
    /// [72. 编辑距离](https://leetcode.cn/problems/edit-distance/)
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = (0..=word2.len() as i32).collect::<Vec<_>>();
        for u1 in word1.bytes() {
            let mut prev = dp[0];
            dp[0] += 1;
            for (j, u2) in word2.bytes().enumerate() {
                let temp = dp[j + 1];
                dp[j + 1] = if u1 == u2 {
                    prev
                } else {
                    std::cmp::min(prev, std::cmp::min(dp[j], dp[j + 1])) + 1
                };
                prev = temp;
            }
        }

        dp[word2.len()]
        // 0ms/2.04MB
    }

    // pub fn min_distance(word1: String, word2: String) -> i32 {
    //     let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];
    //     for j in 1..=word2.len() {
    //         dp[0][j] = dp[0][j - 1] + 1;
    //     }
    //     for i in 1..=word1.len() {
    //         dp[i][0] = dp[i - 1][0] + 1;
    //     }
    //     for i in 0..word1.len() {
    //         for j in 0..word2.len() {
    //             if word1.as_bytes()[i] == word2.as_bytes()[j] {
    //                 dp[i + 1][j + 1] = dp[i][j];
    //             } else {
    //                 dp[i + 1][j + 1] =
    //                     std::cmp::min(dp[i][j], std::cmp::min(dp[i + 1][j], dp[i][j + 1])) + 1;
    //             }
    //         }
    //     }

    //     dp[word1.len()][word2.len()]
    //     // 0ms/3.03MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
