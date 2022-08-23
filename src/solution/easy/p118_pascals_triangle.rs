struct Solution;

impl Solution {
    /// [118. 杨辉三角](https://leetcode.cn/problems/pascals-triangle/)
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle = vec![vec![]; num_rows as usize];

        for i in 0..triangle.len() {
            let mut row = vec![1; i + 1];

            for j in 1..i {
                row[j] = &triangle[i - 1][j - 1] + &triangle[i - 1][j];
            }

            triangle[i] = row;
        }

        triangle
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
