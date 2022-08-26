struct Solution;

impl Solution {
    /// [520. 检测大写字母](https://leetcode.cn/problems/detect-capital/)
    pub fn detect_capital_use(word: String) -> bool {
        if word.len() < 2 {
            return true;
        }

        let bytes = word.as_bytes();
        let upper = bytes[0] < b'a' && bytes[1] < b'a';
        for b in bytes[1..].iter() {
            if upper && b > &b'Z' || (!upper && b < &b'a') {
                return false;
            }
        }

        true
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::detect_capital_use(String::from("USA")));
    }

    #[test]
    fn test2() {
        assert!(!Solution::detect_capital_use(String::from("FlaG")));
    }
}
