struct Solution;

impl Solution {
    /// [131. 分割回文串](https://leetcode.cn/problems/palindrome-partitioning/)
    pub fn partition(s: String) -> Vec<Vec<String>> {
        #[inline]
        fn is_palindrome(s: &[u8]) -> bool {
            for i in 0..s.len() / 2 {
                if s[i] != s[s.len() - 1 - i] {
                    return false;
                }
            }

            true
        }
        fn dfs(s: &[u8], path: &mut Vec<String>) -> Vec<Vec<String>> {
            if s.is_empty() {
                return vec![path.clone()];
            }

            let mut part = Vec::new();
            for i in 0..s.len() {
                let t = &s[..i + 1];
                if is_palindrome(t) {
                    path.push(String::from_utf8_lossy(t).to_string());
                    part.append(&mut dfs(&s[i + 1..], path));
                    path.pop();
                }
            }

            part
        }

        dfs(s.as_bytes(), &mut Vec::new())
        // 70ms/20.04MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::partition("a".to_string()),
            vec![vec!["a".to_string()]]
        );
    }
}
