struct Solution;

impl Solution {
    /// [6. N 字形变换](https://leetcode.cn/problems/zigzag-conversion/)
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows as usize >= s.len() {
            return s;
        }

        let (s, n, l) = (
            s.as_bytes(),
            num_rows as usize,
            (num_rows + num_rows - 2) as usize,
        );
        let mut c = Vec::with_capacity(s.len());
        for i in 0..n {
            let mut j = 0;
            while l * j + i < s.len() {
                c.push(s[l * j + i]);
                if i > 0 && i < n - 1 && l * (j + 1) - i < s.len() {
                    c.push(s[l * (j + 1) - i]);
                }
                j += 1;
            }
        }

        String::from_utf8_lossy(c.as_slice()).into_owned()
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}
