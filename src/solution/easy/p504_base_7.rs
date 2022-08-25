struct Solution;

impl Solution {
    /// [504. 七进制数](https://leetcode.cn/problems/base-7/)
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }

        let neg = num < 0;
        num = num.abs();

        let mut base7 = Vec::new();
        while num > 0 {
            base7.push((b'0' + (num % 7) as u8) as char);
            num /= 7;
        }
        if neg {
            base7.push('-');
        }

        base7.iter().rev().collect()
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_to_base7(100), String::from("202"));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::convert_to_base7(-7), String::from("-10"));
    }
}
