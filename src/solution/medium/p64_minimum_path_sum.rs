struct Solution;

impl Solution {
    /// [64. 最小路径和](https://leetcode.cn/problems/minimum-path-sum/)
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 1..grid.len() {
            grid[i][0] += grid[i - 1][0];
        }
        for j in 1..grid[0].len() {
            grid[0][j] += grid[0][j - 1];
        }
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1]);
            }
        }

        grid[grid.len() - 1][grid[0].len() - 1]
        // 4ms/2.5MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
