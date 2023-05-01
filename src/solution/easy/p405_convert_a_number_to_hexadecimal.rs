struct Solution;

impl Solution {
    /// [405. 数字转换为十六进制数](https://leetcode.cn/problems/convert-a-number-to-hexadecimal/)
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }

        let (mut num, mut hex_vec) = (num as i64, Vec::new());
        if num < 0 {
            num += 1 << 32;
        }

        while num != 0 {
            hex_vec.push(match num & 15 {
                10..=15 => (num as u8 & 15) - 10 + b'a',
                _ => (num as u8 & 15) + b'0',
            } as char);

            num >>= 4;
        }

        hex_vec.iter().rev().collect::<String>()
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::to_hex(26), String::from("1a"));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::to_hex(-1), String::from("ffffffff"));
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::to_hex(0), String::from("0"));
    }
}
