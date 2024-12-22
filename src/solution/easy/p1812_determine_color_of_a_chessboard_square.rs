struct Solution;

impl Solution {
    /// [1812. 判断国际象棋棋盘中一个格子的颜色](https://leetcode.cn/problems/determine-color-of-a-chessboard-square/)
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.as_bytes()[0] & 1 != coordinates.as_bytes()[1] & 1
        // 0ms/2.24MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::square_is_white(String::from("a1")));
    }

    #[test]
    fn test2() {
        assert!(Solution::square_is_white(String::from("h3")));
    }

    #[test]
    fn test3() {
        assert!(!Solution::square_is_white(String::from("c7")));
    }
}
