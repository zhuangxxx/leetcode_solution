struct Solution;

impl Solution {
    /// [3001. 捕获黑皇后需要的最少移动次数](https://leetcode.cn/problems/minimum-moves-to-capture-the-queen/)
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if a == e {
            if a != c || d < std::cmp::min(b, f) || d > std::cmp::max(b, f) {
                1
            } else {
                2
            }
        } else if b == f {
            if b != d || c < std::cmp::min(a, e) || c > std::cmp::max(a, e) {
                1
            } else {
                2
            }
        } else if (c - e).abs() == (d - f).abs() {
            if (a - c).abs() != (b - d).abs()
                || (c - a).is_positive() ^ (a - e).is_positive()
                || (d - b).is_positive() ^ (b - f).is_positive()
                || (a - c).abs() >= (c - e).abs()
            {
                1
            } else {
                2
            }
        } else {
            2
        }
        // 0ms/2.26MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(1, 1, 8, 8, 2, 3),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(5, 3, 3, 4, 5, 2),
            1
        );
    }
}
