struct Solution;

impl Solution {
    /// [392. 判断子序列](https://leetcode.cn/problems/is-subsequence/)
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut sp, mut tp) = (0, 0);
        let (s, t) = (s.as_bytes(), t.as_bytes());

        while sp < s.len() && tp < t.len() {
            if s[sp] == t[tp] {
                sp += 1;
            }

            tp += 1;
        }

        sp == s.len()
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn trap1() {
        assert!(Solution::is_subsequence(String::new(), String::new()));
    }

    #[test]
    fn trap2() {
        assert!(!Solution::is_subsequence(
            "abcde".to_string(),
            "abc".to_string()
        ));
    }
}
