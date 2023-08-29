struct Solution;

impl Solution {
    /// [97. 交错字符串](https://leetcode.cn/problems/interleaving-string/)
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (b1, b2, b3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![false; b2.len() + 1];

        if b1.len() + b2.len() != b3.len() {
            return false;
        }

        dp[0] = true;
        for i1 in 0..=b1.len() {
            for i2 in 0..=b2.len() {
                if i1 > 0 {
                    dp[i2] &= (b1[i1 - 1] == b3[i1 + i2 - 1]);
                }
                if i2 > 0 {
                    dp[i2] |= (dp[i2 - 1] && b2[i2 - 1] == b3[i1 + i2 - 1]);
                }
            }
        }

        dp[b2.len()]
        // 0ms/1.83MB
    }

    // pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    //     let (l1, l2, l3) = (s1.len(), s2.len(), s3.len());
    //     let (b1, b2, b3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
    //     let mut dp = vec![vec![false; l2 + 1]; l1 + 1];

    //     if l1 + l2 != l3 {
    //         return false;
    //     }

    //     dp[0][0] = true;
    //     for i1 in 0..=l1 {
    //         for i2 in 0..=l2 {
    //             if i1 > 0 {
    //                 dp[i1][i2] |= (dp[i1 - 1][i2] && b1[i1 - 1] == b3[i1 + i2 - 1]);
    //             }
    //             if i2 > 0 {
    //                 dp[i1][i2] |= (dp[i1][i2 - 1] && b2[i2 - 1] == b3[i1 + i2 - 1]);
    //             }
    //         }
    //     }

    //     dp[l1][l2]
    //     // 0ms/2.13MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_interleave(
            String::new(),
            String::new(),
            String::new()
        ));
    }

    #[test]
    fn trap1() {
        assert!(!Solution::is_interleave(
            "a".to_string(),
            "b".to_string(),
            "a".to_string()
        ));
    }

    #[test]
    fn time1() {
        assert!(!Solution::is_interleave(
            "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(),
            "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string(),
            "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string()
        ));
    }
}
