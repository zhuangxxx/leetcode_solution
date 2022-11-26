struct Solution;

impl Solution {
    /// [766. 托普利茨矩阵](https://leetcode.cn/problems/toeplitz-matrix/)
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for y in 1..matrix.len() {
            for x in 1..matrix[0].len() {
                if matrix[y][x] != matrix[y - 1][x - 1] {
                    return false;
                }
            }
        }

        true
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]));
    }
}
