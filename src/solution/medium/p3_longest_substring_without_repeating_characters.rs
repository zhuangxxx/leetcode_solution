struct Solution;

impl Solution {
    /// [3. 无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/)
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let s = s.as_bytes();

        let (mut pos, mut map) = (0, std::collections::HashMap::new());
        for i in 0..s.len() {
            let old = map.insert(s[i], i);
            if let Some(old) = old {
                if old >= pos {
                    pos = old + 1;
                }
            }

            if i - pos + 1 > max {
                max = i - pos + 1;
            }
        }

        (if pos == 0 { s.len() } else { max }) as i32
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::length_of_longest_substring(String::new()), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::length_of_longest_substring("au".to_string()), 2);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::length_of_longest_substring("aa".to_string()), 1);
    }
}
