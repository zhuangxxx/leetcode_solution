struct Solution;

impl Solution {
    /// [91. 解码方法](https://leetcode.cn/problems/decode-ways/)
    pub fn num_decodings(s: String) -> i32 {
        let (mut num, mut val) = ([0, 1], 1);
        let mut prev = '0';
        for c in s.chars() {
            if c == '0' && !('1'..='2').contains(&prev) {
                return 0;
            }

            if c == '0' {
                num[0] = num[1];
                num[1] = 0;
            } else if prev == '2' && ('0'..='6').contains(&c) || prev == '1' {
                num[0] += num[1];
                num.swap(0, 1);
            } else {
                val *= num[0] + num[1];
                num = [0, 1];
            }
            prev = c;
        }

        val * (num[0] + num[1])
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_decodings(String::from("12")), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_decodings(String::from("226")), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_decodings(String::from("06")), 0);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::num_decodings(String::from("222202222")), 15);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::num_decodings(String::from("222272222")), 25);
    }

    #[test]
    fn trap3() {
        assert_eq!(Solution::num_decodings(String::from("10")), 1);
    }
}
