struct Solution;

impl Solution {
    /// [5. 最长回文子串](https://leetcode.cn/problems/longest-palindromic-substring/)
    ///
    /// Expand Around Center
    ///
    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(1)
    pub fn longest_palindrome(s: String) -> String {
        fn expand(s: &[u8], mut l: usize, mut r: usize) -> (usize, usize) {
            if l != r && s[l] != s[r] {
                return (l, l);
            }

            while l > 0 && r < s.len() - 1 {
                l -= 1;
                r += 1;
                if s[l] != s[r] {
                    l += 1;
                    r -= 1;
                    break;
                }
            }

            (l, r)
        }

        let s = s.as_bytes().to_vec();
        let (mut b, mut e) = (0, 0);
        for i in 0..s.len() - 1 {
            if s.len() - i - 1 << 1 < e - b {
                break;
            }
            if i << 1 < e - b {
                continue;
            }
            let (l, r) = std::cmp::max_by(
                expand(&s, i, i),
                expand(&s, i, i + 1),
                |(l1, r1), (l2, r2)| (r1 - l1).cmp(&(r2 - l2)),
            );
            if r - l > e - b {
                b = l;
                e = r;
            }
        }

        String::from_utf8_lossy(&s[b..=e]).into_owned()
        // 4ms/2.2MB
    }

    // /// Dynamic Programming
    // ///
    // /// Time Complexity: O(n^2)
    // ///
    // /// Space Complexity: O(n^2)
    // pub fn longest_palindrome(s: String) -> String {
    //     let s = s.as_bytes().to_vec();
    //     let (mut b, mut e) = (0, 0);
    //     let mut dp = vec![vec![false; s.len()]; s.len()];
    //     for i in 0..s.len() {
    //         dp[i][i] = true;
    //     }

    //     for j in 1..s.len() {
    //         for i in 0..j {
    //             if s[i] != s[j] {
    //                 dp[i][j] = false;
    //             } else if j - i < 3 || dp[i + 1][j - 1] {
    //                 dp[i][j] = true;
    //                 if j - i > e - b {
    //                     b = i;
    //                     e = j;
    //                 }
    //             }
    //         }
    //     }

    //     String::from_utf8_lossy(&s[b..=e]).into_owned()
    //     // 4ms/2.2MB
    // }

    // /// Violent
    // ///
    // /// Time Complexity: O(n^3)
    // ///
    // /// Space Complexity: O(1)
    // pub fn longest_palindrome(s: String) -> String {
    //     fn is_palindrome(s: &[u8], mut l: usize, mut r: usize) -> bool {
    //         while l < r {
    //             if s[l] != s[r] {
    //                 return false;
    //             }
    //             l += 1;
    //             r -= 1;
    //         }

    //         true
    //     }
    //     let s = s.as_bytes().to_vec();
    //     let (mut b, mut l) = (0, 1);
    //     for i in 0..s.len() - 1 {
    //         for j in i + 1..s.len() {
    //             if j - i + 1 > l && is_palindrome(&s, i, j) {
    //                 b = i;
    //                 l = j - i + 1;
    //             }
    //         }
    //     }

    //     String::from_utf8_lossy(&s[b..b + l]).into_owned()
    //     // 40ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
    }
}
