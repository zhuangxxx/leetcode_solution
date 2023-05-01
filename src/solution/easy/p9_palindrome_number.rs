struct Solution;

impl Solution {
    /// [9. 回文数](https://leetcode-cn.com/problems/palindrome-number/)
    pub fn is_palindrome(x: i32) -> bool {
        let mut y = x;
        let mut z = 0;
        while y > 0 {
            z = z * 10 + y % 10;
            y /= 10;
        }

        x == z
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_palindrome(-101));
    }
}
