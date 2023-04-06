struct Solution;

impl Solution {
    /// [62. 不同路径](https://leetcode.cn/problems/unique-paths/)
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut num = 1i64;
        let (mut x, mut y) = (n as i64, 1);
        while y < m as i64 {
            num = num * x / y;
            x += 1;
            y += 1;
        }

        num as i32
        // 0ms/2.1MB
    }

    // pub fn unique_paths(m: i32, n: i32) -> i32 {
    //     let (m, n) = (m as usize, n as usize);
    //     let mut paths = vec![vec![1; n]; m];
    //     for i in 1..m {
    //         for j in 1..n {
    //             paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
    //         }
    //     }

    //     paths[m - 1][n - 1]
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::unique_paths(51, 9), 1916797311);
    }
}
