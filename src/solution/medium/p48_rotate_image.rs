struct Solution;

impl Solution {
    /// [48. 旋转图像](https://leetcode.cn/problems/rotate-image/)
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i..n {
                let temp = matrix[j][i];
                matrix[j][i] = matrix[i][j];
                matrix[i][j] = temp;
            }
        }

        matrix.iter_mut().for_each(|column| column.reverse());
        // 0ms/2MB
    }

    // pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    //     let n = matrix.len();
    //     for i in 0..n / 2 {
    //         for j in i..n - i - 1 {
    //             let temp = matrix[i][j];
    //             matrix[i][j] = matrix[n - j - 1][i];
    //             matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
    //             matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
    //             matrix[j][n - i - 1] = temp;
    //         }
    //     }
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn test2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
