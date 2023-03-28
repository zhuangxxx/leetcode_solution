struct Solution;

impl Solution {
    /// [54. 螺旋矩阵](https://leetcode.cn/problems/spiral-matrix/)
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ord = Vec::new();

        let (mut t, mut b, mut l, mut r) = (0, matrix.len() - 1, 0, matrix[0].len() - 1);
        while t <= b && l <= r {
            for j in l..=r {
                ord.push(matrix[t][j]);
            }
            if t == b {
                break;
            }
            t += 1;
            for i in t..=b {
                ord.push(matrix[i][r]);
            }
            if l == r {
                break;
            }
            r -= 1;
            for j in (l..=r).rev() {
                ord.push(matrix[b][j]);
            }
            b -= 1;
            for i in (t..=b).rev() {
                ord.push(matrix[i][l]);
            }
            l += 1;
        }

        ord
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3], vec![4]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![1, 2, 3, 6, 5, 4]
        );
    }
}
