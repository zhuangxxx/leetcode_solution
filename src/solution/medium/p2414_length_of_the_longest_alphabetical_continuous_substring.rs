struct Solution;

impl Solution {
    /// [2414. 最长的字母序连续子字符串的长度](https://leetcode.cn/problems/length-of-the-longest-alphabetical-continuous-substring/)
    pub fn longest_continuous_substring(s: String) -> i32 {
        let (mut len, mut max) = (0, 0);

        let mut p = 0;
        for (i, b) in s.bytes().enumerate() {
            if b == p + 1 {
                len += 1;
            } else {
                if len > max {
                    max = len;
                }
                len = 1;
            }
            p = b;
        }

        if len > max {
            len
        } else {
            max
        }
        // 2ms/2.37MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_continuous_substring("abacaba".to_string()),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_continuous_substring("abcde".to_string()),
            5
        );
    }
}
