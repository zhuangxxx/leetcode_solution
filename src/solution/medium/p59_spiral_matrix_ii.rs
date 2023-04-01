struct Solution;

impl Solution {
    /// [59. 螺旋矩阵 II](https://leetcode.cn/problems/spiral-matrix-ii/)
    pub fn generate_matrix(mut n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![1; n as usize]; n as usize];
        if n == 1 {
            return matrix;
        }

        let (mut t, mut b, mut l, mut r) = (0, n as usize - 1, 0, n as usize - 1);
        n = 1;
        while t <= b && l <= r {
            for j in l..=r {
                matrix[t][j] = n;
                n += 1;
            }
            t += 1;
            for i in t..=b {
                matrix[i][r] = n;
                n += 1;
            }
            r -= 1;
            for j in (l..=r).rev() {
                matrix[b][j] = n;
                n += 1;
            }
            b -= 1;
            for i in (t..=b).rev() {
                matrix[i][l] = n;
                n += 1;
            }
            l += 1;
        }

        matrix
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
