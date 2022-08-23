struct Solution;

impl Solution {
    /// [263. 丑数](https://leetcode.cn/problems/ugly-number/)
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;

        while n > 1 {
            if n & 1 == 0 {
                n /= 2;
            } else if n % 3 == 0 {
                n /= 3;
            } else if n % 5 == 0 {
                n /= 5;
            } else {
                break;
            }
        }

        n == 1
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_ugly(6));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_ugly(14));
    }
}
