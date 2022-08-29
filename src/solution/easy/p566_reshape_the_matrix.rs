struct Solution;

impl Solution {
    /// [566. 重塑矩阵](https://leetcode.cn/problems/reshape-the-matrix/)
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (n, m) = (mat.len(), mat[0].len());
        if n * m != (r * c) as usize {
            return mat;
        }

        let mut res = vec![vec![0; c as usize]; r as usize];
        for i in 0..n * m {
            res[i / c as usize][i % c as usize] = mat[i / m][i % m];
        }

        res
        // 8ms/2.2MB
    }

    // pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    //     if mat.len() * mat[0].len() != (r * c) as usize {
    //         return mat;
    //     }

    //     let mut res = vec![vec![0; c as usize]; r as usize];
    //     let (mut i, mut j) = (0, 0);
    //     for row in mat {
    //         for col in row {
    //             res[i][j] = col;

    //             if j < c as usize - 1 {
    //                 j += 1;
    //             } else {
    //                 i += 1;
    //                 j = 0;
    //             }
    //         }
    //     }

    //     res
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
    }
}
