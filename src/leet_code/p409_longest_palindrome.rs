struct Solution;

impl Solution {
    /// [409. 最长回文串](https://leetcode.cn/problems/longest-palindrome/)
    pub fn longest_palindrome(s: String) -> i32 {
        let (mut sum, mut bytes) = (0, [0; 128]);

        for u in s.bytes() {
            bytes[u as usize] += 1;
        }

        for n in bytes {
            if n & 1 == 0 || sum & 1 == 0 {
                sum += n;
            } else {
                sum += n - 1;
            }
        }

        sum
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome(String::from("abccccdd")), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome(String::from("a")), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_palindrome(String::from("bb")), 2);
    }
}
