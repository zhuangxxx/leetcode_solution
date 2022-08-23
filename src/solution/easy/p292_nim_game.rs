struct Solution;

impl Solution {
    /// [292. Nim 游戏](https://leetcode.cn/problems/nim-game/)
    pub fn can_win_nim(n: i32) -> bool {
        n & 3 != 0
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::can_win_nim(4));
    }

    #[test]
    fn test2() {
        assert!(Solution::can_win_nim(1));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_win_nim(2));
    }
}
