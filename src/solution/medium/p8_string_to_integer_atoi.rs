struct Solution;

impl Solution {
    /// [8. 字符串转换整数 (atoi)](https://leetcode.cn/problems/string-to-integer-atoi/)
    pub fn my_atoi(s: String) -> i32 {
        const MAX_DIV_10: i32 = i32::MAX / 10;

        let (mut n, mut i) = (0, 0);
        for c in s.chars() {
            if n != 0 && !c.is_ascii_digit() {
                break;
            }
            match c {
                '+' => n = 1,
                '-' => n = -1,
                '0'..='9' => {
                    if n == 0 {
                        n = 1;
                    }
                    let d = (c as u8 - b'0') as i32;
                    if i < MAX_DIV_10 || (i == MAX_DIV_10 && d <= 7) {
                        i = i * 10 + d;
                    } else {
                        i = if n == 1 { i32::MAX } else { i32::MIN };
                        n = 1;
                        break;
                    }
                }
                ' ' => continue,
                _ => break,
            }
        }

        i * n
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::my_atoi("+-1".to_string()), 0);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::my_atoi(" -2147483648.f32".to_string()), i32::MIN);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::my_atoi("21474836460".to_string()), i32::MAX);
    }
}
