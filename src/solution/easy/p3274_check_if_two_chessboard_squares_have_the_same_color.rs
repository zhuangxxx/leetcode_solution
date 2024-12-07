struct Solution;

impl Solution {
    /// [3274. 检查棋盘方格颜色是否相同](https://leetcode.cn/problems/check-if-two-chessboard-squares-have-the-same-color/)
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        coordinate1.as_bytes()[0] as i32 + coordinate1.as_bytes()[1] as i32
            - coordinate2.as_bytes()[0] as i32
            - coordinate2.as_bytes()[1] as i32
            & 1
            == 0

        // 1ms/2.32MB
    }

    // pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    //     let (c1, c2) = (coordinate1.as_bytes(), coordinate2.as_bytes());
    //     !((c1[0] & 1 == c2[0] & 1) ^ (c1[1] & 1 == c2[1] & 1))

    //     // 1ms/2.33MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_two_chessboards(
            String::from("a1"),
            String::from("c3")
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::check_two_chessboards(
            String::from("a1"),
            String::from("h3")
        ));
    }
}
