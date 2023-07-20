struct Solution;

impl Solution {
    /// [1025. 除数博弈](https://leetcode.cn/problems/divisor-game/)
    pub fn divisor_game(n: i32) -> bool {
        n & 1 == 0
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::divisor_game(2));
    }

    #[test]
    fn test2() {
        assert!(!Solution::divisor_game(3));
    }
}
