struct Solution;

impl Solution {
    /// [867. 转置矩阵](https://leetcode.cn/problems/transpose-matrix/)
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut t = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                t[j][i] = matrix[i][j];
            }
        }

        t
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }
}
