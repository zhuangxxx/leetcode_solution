struct Solution;

impl Solution {
    /// [63. 不同路径 II](https://leetcode.cn/problems/unique-paths-ii/)
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        obstacle_grid = obstacle_grid
            .into_iter()
            .map(|obstacle_line| obstacle_line.into_iter().map(|v| v ^ 1).collect())
            .collect();

        for i in 1..obstacle_grid.len() {
            if obstacle_grid[i][0] != 0 {
                obstacle_grid[i][0] = obstacle_grid[i - 1][0];
            }
        }
        for j in 1..obstacle_grid[0].len() {
            if obstacle_grid[0][j] != 0 {
                obstacle_grid[0][j] = obstacle_grid[0][j - 1];
            }
        }
        for i in 1..obstacle_grid.len() {
            for j in 1..obstacle_grid[0].len() {
                if obstacle_grid[i][j] != 0 {
                    obstacle_grid[i][j] = obstacle_grid[i - 1][j] + obstacle_grid[i][j - 1];
                }
            }
        }

        obstacle_grid[obstacle_grid.len() - 1][obstacle_grid[0].len() - 1]
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1, 0, 0]]),
            0
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![1], vec![0], vec![0], vec![0]]),
            0
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            0
        );
    }

    #[test]
    fn fail4() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0],]), 1);
    }
}
