struct Solution;

impl Solution {
    /// [125. 验证回文串](https://leetcode.cn/problems/valid-palindrome/)
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();

        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            while i < j
                && !s[i].is_ascii_digit()
                && !s[i].is_ascii_lowercase()
                && !s[i].is_ascii_uppercase()
            {
                i += 1;
            }

            while i < j
                && !s[j].is_ascii_digit()
                && !s[j].is_ascii_lowercase()
                && !s[j].is_ascii_uppercase()
            {
                j -= 1;
            }

            if i < j {
                if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
                    return false;
                }

                i += 1;
                j -= 1;
            }
        }

        true
        // 0ms/2.6MB
    }

    // pub fn is_palindrome(s: String) -> bool {
    //     let s = s
    //         .chars()
    //         .map(|c| {
    //             let u = c as u8;
    //             if (u >= 'a' as u8 && u <= 'z' as u8) || (u >= '0' as u8 && u <= '9' as u8) {
    //                 u
    //             } else if u >= 'A' as u8 && u <= 'Z' as u8 {
    //                 u + 32
    //             } else {
    //                 0
    //             }
    //         })
    //         .filter(|&u| u != 0)
    //         .collect::<Vec<_>>();

    //     if s.len() == 0 {
    //         return true;
    //     }

    //     let (mut i, mut j) = (0, s.len() - 1);
    //     while i < j {
    //         if s[i] != s[j] {
    //             return false;
    //         }

    //         i += 1;
    //         j -= 1;
    //     }

    //     true
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn fail1() {
        assert!(Solution::is_palindrome(" ".to_string()));
    }

    #[test]
    fn fail2() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }

    #[test]
    fn fail3() {
        assert!(Solution::is_palindrome("a.".to_string()));
    }
}
