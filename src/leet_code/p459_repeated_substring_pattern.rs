struct Solution;

// TODO: Implement KMP Algorithm
impl Solution {
    /// [459. 重复的子字符串](https://leetcode.cn/problems/repeated-substring-pattern/)
    pub fn repeated_substring_pattern(s: String) -> bool {
        let (mut i, len) = (1, s.len());

        while i < len {
            if s[0..len - i] == s[i..len] && s[0..i] == s[len - i..len] {
                return true;
            }
            i += 1;
        }

        false
        // 0ms/2.2MB
    }

    // pub fn repeated_substring_pattern(s: String) -> bool {
    //     (s[1..].to_string() + s[..s.len() - 1].to_string().as_str()).contains(s.as_str())
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::repeated_substring_pattern(
            "abcabcabcabc".to_string()
        ));
    }

    #[test]
    fn fail1() {
        assert!(Solution::repeated_substring_pattern("bb".to_string()))
    }
}
