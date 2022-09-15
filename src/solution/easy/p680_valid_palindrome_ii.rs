struct Solution;

impl Solution {
    /// [680. 验证回文串 II](https://leetcode.cn/problems/valid-palindrome-ii/)
    pub fn valid_palindrome(s: String) -> bool {
        fn check_palindrome(b: &[u8], mut l: usize, mut r: usize) -> bool {
            while l < r {
                if b[l] != b[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }

            true
        }

        let b = s.as_bytes();

        let (mut l, mut r) = (0, b.len() - 1);
        while l < r {
            if b[l] != b[r] {
                return check_palindrome(b, l, r - 1) || check_palindrome(b, l + 1, r);
            }
            l += 1;
            r -= 1;
        }

        true
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::valid_palindrome(String::from("aba")));
    }

    #[test]
    fn test2() {
        assert!(Solution::valid_palindrome(String::from("abca")));
    }

    #[test]
    fn test3() {
        assert!(!Solution::valid_palindrome(String::from("abc")));
    }
}
