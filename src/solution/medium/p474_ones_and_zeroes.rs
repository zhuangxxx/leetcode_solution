struct Solution;

impl Solution {
    /// [474. 一和零](https://leetcode.cn/problems/ones-and-zeroes/)
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for (x, y) in strs.into_iter().map(|s| {
            s.bytes().fold(
                (0, 0),
                |(m, n), b| if b == b'0' { (m + 1, n) } else { (m, n + 1) },
            )
        }) {
            for i in (0..=m).rev() {
                for j in (0..=n).rev() {
                    if i >= x && j >= y {
                        dp[i][j] = std::cmp::max(dp[i][j], dp[i - x][j - y] + 1);
                    }
                }
            }
        }

        dp[m][n]
        // 22ms/2.22MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_string(),
                    "0001".to_string(),
                    "111001".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                5,
                3
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_max_form(
                vec!["10".to_string(), "0".to_string(), "1".to_string()],
                1,
                1
            ),
            2
        );
    }
}
