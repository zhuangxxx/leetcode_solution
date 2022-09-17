struct Solution;

impl Solution {
    /// [696. 计数二进制子串](https://leetcode.cn/problems/count-binary-substrings/)
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut count = 0;

        let (mut f, mut new, mut old) = (b' ', 0, 0);
        for b in s.bytes() {
            if b == f {
                new += 1;
                if new <= old {
                    count += 1;
                }
            } else {
                if f != b' ' {
                    count += 1;
                }
                f = b;
                old = new;
                new = 1;
            }
        }

        count
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    }
}
