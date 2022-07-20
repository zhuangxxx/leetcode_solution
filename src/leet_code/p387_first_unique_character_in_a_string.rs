struct Solution;

impl Solution {
    /// [387. 字符串中的第一个唯一字符](https://leetcode.cn/problems/first-unique-character-in-a-string/)
    pub fn first_uniq_char(s: String) -> i32 {
        let mut bytes = [0; 26];

        let s = s.as_bytes();
        for &u in s {
            bytes[(u - b'a') as usize] += 1;
        }

        for i in 0..s.len() {
            unsafe {
                if bytes[(s[i] - b'a') as usize] == 1 {
                    return i as i32;
                }
            }
        }

        -1
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
