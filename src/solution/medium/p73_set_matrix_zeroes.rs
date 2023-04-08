struct Solution;

impl Solution {
    /// [73. 矩阵置零](https://leetcode.cn/problems/set-matrix-zeroes/)
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col0 = false;
        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                col0 = true;
            }
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in (0..matrix.len()).rev() {
            for j in 1..matrix[0].len() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if col0 {
                matrix[i][0] = 0;
            }
        }
        // 4ms/2.4MB
    }

    // pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    //     let (mut col0, mut row0) = (false, false);
    //     for i in 0..matrix.len() {
    //         if matrix[0] == 0 {
    //             col0 = true;
    //             break;
    //         }
    //     }
    //     for j in 0..matrix[0].len() {
    //         if matrix[0][j] == 0 {
    //             row0 = true;
    //             break;
    //         }
    //     }
    //     for i in 1..matrix.len() {
    //         for j in 1..matrix[0].len() {
    //             if matrix[i][j] == 0 {
    //                 matrix[i][0] = 0;
    //                 matrix[0][j] = 0;
    //             }
    //         }
    //     }
    //     for i in 1..matrix.len() {
    //         if matrix[i][0] != 0 {
    //             continue;
    //         }
    //         for j in 0..matrix[0].len() {
    //             matrix[i][j] = 0;
    //         }
    //     }
    //     for j in 1..matrix[0].len() {
    //         if matrix[0][j] != 0 {
    //             continue;
    //         }
    //         for i in 0..matrix.len() {
    //             matrix[i][j] = 0;
    //         }
    //     }
    //     if col0 {
    //         for i in 0..matrix.len() {
    //             matrix[i][0] = 0;
    //         }
    //     }
    //     if row0 {
    //         for j in 0..matrix[0].len() {
    //             matrix[0][j] = 0;
    //         }
    //     }
    //     // 4ms/2.3MB
    // }

    // pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    //     let mut xy = Vec::new();
    //     for i in 0..matrix.len() {
    //         for j in 0..matrix[0].len() {
    //             if matrix[i][j] == 0 {
    //                 xy.push((i, j));
    //             }
    //         }
    //     }

    //     for (y, x) in xy {
    //         for j in 0..matrix[0].len() {
    //             matrix[y][j] = 0;
    //         }
    //         for i in 0..matrix.len() {
    //             matrix[i][x] = 0;
    //         }
    //     }
    //     // 8ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut result);
        assert_eq!(result, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test2() {
        let mut result = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut result);
        assert_eq!(
            result,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
