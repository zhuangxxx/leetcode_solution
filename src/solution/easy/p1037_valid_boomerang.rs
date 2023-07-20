struct Solution;

impl Solution {
    /// [1037. 有效的回旋镖](https://leetcode.cn/problems/valid-boomerang/)
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        (points[1][0] - points[0][0]) * (points[2][1] - points[1][1])
            != (points[2][0] - points[1][0]) * (points[1][1] - points[0][1])
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 3],
            vec![3, 2]
        ]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_boomerang(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3]
        ]));
    }
}
