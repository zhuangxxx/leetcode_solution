pub struct Solution;

impl Solution {
    /// [58. 最后一个单词的长度](https://leetcode.cn/problems/length-of-last-word/)
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut len, mut i) = (0, s.len() - 1);

        while s[i] == ' ' as u8 {
            if i == 0 {
                break;
            }
            i -= 1;
        }

        while s[i] != ' ' as u8 {
            len += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }

        len
        // 0ms/2MB
    }

    // pub fn length_of_last_word(s: String) -> i32 {
    //     let mut len = 0;
    //     let mut prev = ' ';

    //     for c in s.chars() {
    //         if c >= 'A' && c <= 'z' {
    //             if prev == ' ' {
    //                 len = 0;
    //             }

    //             len += 1;
    //         }

    //         prev = c;
    //     }

    //     len
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1)
    }
}
