struct Solution;

impl Solution {
    /// [482. 密钥格式化](https://leetcode.cn/problems/license-key-formatting/)
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let (mut key, mut len) = (Vec::new(), 0);

        for u in s.bytes().rev() {
            if len > 0 && len % k == 0 && key.last() != Some(&'-') {
                key.push('-');
            }

            match u {
                b'a'..=b'z' => key.push((u - 32) as char),
                b'A'..=b'Z' | b'0'..=b'9' => key.push(u as char),
                _ => continue,
            }

            len += 1;
        }

        if key.last() == Some(&'-') {
            key.pop();
        }

        key.iter().rev().collect()
        // 0ms/2.5MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
            "5F3Z-2E9W".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::license_key_formatting("2-5g-3-J".to_string(), 2),
            "2-5G-3J".to_string()
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::license_key_formatting("--a-a-a-a--".to_string(), 2),
            "AA-AA".to_string()
        )
    }
}
