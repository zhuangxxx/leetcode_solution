struct Solution;

impl Solution {
    /// [709. 转换成小写字母](https://leetcode.cn/problems/to-lower-case/)
    pub fn to_lower_case(s: String) -> String {
        s.bytes()
            .map(|u| if u >= b'A' && u <= b'Z' { u | 32 } else { u } as char)
            .collect()
        // 0ms/2.2MB
    }

    // pub fn to_lower_case(s: String) -> String {
    //     s.bytes()
    //         .map(|u| if u >= b'A' && u <= b'Z' { u + 32 } else { u } as char)
    //         .collect()
    //     // 0ms/1.9MB
    // }

    // pub fn to_lower_case(s: String) -> String {
    //     s.to_ascii_lowercase()
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::to_lower_case("Hello".to_string()),
            "hello".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::to_lower_case("here".to_string()),
            "here".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::to_lower_case("LOVELY".to_string()),
            "lovely".to_string()
        );
    }
}
