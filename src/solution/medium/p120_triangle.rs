struct Solution;

impl Solution {
    /// [120. 三角形最小路径和](https://leetcode.cn/problems/triangle/)
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in (0..triangle.len() - 1).rev() {
            for j in 0..triangle[i].len() {
                triangle[i][j] += std::cmp::min(triangle[i + 1][j], triangle[i + 1][j + 1]);
            }
        }

        triangle[0][0]
        // 0ms/2.18MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
