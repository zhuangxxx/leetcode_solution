struct Solution;

impl Solution {
    /// [693. 交替位二进制数](https://leetcode.cn/problems/binary-number-with-alternating-bits/)
    pub fn has_alternating_bits(n: i32) -> bool {
        (n >> 1 ^ n) & (n >> 1 ^ n) + 1 == 0
        // 0ms/2MB
    }

    // pub fn has_alternating_bits(n: i32) -> bool {
    //     (n >> 1) + n & (n >> 1) + n + 1 == 0
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::has_alternating_bits(5));
    }

    #[test]
    fn test2() {
        assert!(!Solution::has_alternating_bits(7));
    }

    #[test]
    fn test3() {
        assert!(!Solution::has_alternating_bits(11));
    }
}
