struct Solution;

impl Solution {
    /// [74. 搜索二维矩阵](https://leetcode.cn/problems/search-a-2d-matrix/)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix[0][0] > target || matrix[matrix.len() - 1][matrix[0].len() - 1] < target {
            return false;
        } else if matrix[0][0] == target || matrix[matrix.len() - 1][matrix[0].len() - 1] == target
        {
            return true;
        }

        let (mut l, mut r) = (0, matrix.len() * matrix[0].len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if matrix[m / matrix[0].len()][m % matrix[0].len()] < target {
                l = m + 1;
            } else if matrix[m / matrix[0].len()][m % matrix[0].len()] > target {
                r = m;
            } else {
                return true;
            }
        }

        false
        // 0ms/2.2MB
    }

    // pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    //     let (m, n) = (matrix.len() - 1, matrix[0].len() - 1);
    //     if matrix[0][0] > target || matrix[m][n] < target {
    //         return false;
    //     } else if matrix[0][0] == target || matrix[m][n] == target {
    //         return true;
    //     }

    //     let (mut l, mut r) = (0, m);
    //     while l < r {
    //         let mid = l + (r - l) / 2;
    //         if matrix[mid][n] < target {
    //             l = mid + 1;
    //         } else if matrix[mid][0] > target {
    //             r = mid;
    //         } else {
    //             l = mid;
    //             break;
    //         }
    //     }

    //     if matrix[l][0] == target || matrix[l][n] == target {
    //         return true;
    //     }

    //     let i = l;
    //     l = 0;
    //     r = n;
    //     while l < r {
    //         let mid = l + (r - l) / 2;
    //         if matrix[i][mid] < target {
    //             l = mid + 1;
    //         } else if matrix[i][mid] > target {
    //             r = mid;
    //         } else {
    //             return true;
    //         }
    //     }

    //     false
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }

    #[test]
    fn fail1() {
        assert!(Solution::search_matrix(vec![vec![1]], 1));
    }

    #[test]
    fn fail2() {
        assert!(Solution::search_matrix(vec![vec![1], vec![3], vec![5]], 3));
    }
}
