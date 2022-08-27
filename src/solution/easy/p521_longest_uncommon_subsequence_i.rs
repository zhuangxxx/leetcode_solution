struct Solution;

impl Solution {
    /// [521. 最长特殊序列 Ⅰ](https://leetcode.cn/problems/longest-uncommon-subsequence-i/)
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            std::cmp::max(a.len(), b.len()) as i32
        }
        // 4ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_lu_slength(String::from("aba"), String::from("cdc")),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_lu_slength(String::from("aaa"), String::from("bbb")),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_lu_slength(String::from("aaa"), String::from("aaa")),
            -1
        );
    }
}
