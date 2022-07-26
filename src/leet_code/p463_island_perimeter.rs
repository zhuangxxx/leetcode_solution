struct Solution;

impl Solution {
    /// [463. 岛屿的周长](https://leetcode.cn/problems/island-perimeter/)
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut len = 0;

        let (x, y) = (grid[0].len() - 1, grid.len() - 1);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }

                if i == 0 || grid[i - 1][j] == 0 {
                    len += 1;
                }
                if i == y || grid[i + 1][j] == 0 {
                    len += 1;
                }
                if j == 0 || grid[i][j - 1] == 0 {
                    len += 1;
                }
                if j == x || grid[i][j + 1] == 0 {
                    len += 1;
                }
            }
        }

        len
        // 8ms/2.2MB
    }

    // pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    //     fn dfs(i: i32, j: i32, grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    //         if i < 0 || i >= y || j < 0 || j >= x || grid[i as usize][j as usize] == 0 {
    //             return 1;
    //         }

    //         if grid[i as usize][j as usize] == -1 {
    //             return 0;
    //         }

    //         grid[i as usize][j as usize] = -1;

    //         let mut len = 0;
    //         len += dfs(i - 1, j, grid, x, y);
    //         len += dfs(i + 1, j, grid, x, y);
    //         len += dfs(i, j - 1, grid, x, y);
    //         len += dfs(i, j + 1, grid, x, y);

    //         len
    //     }

    //     let mut len = 0;

    //     let mut grid = grid;
    //     let (x, y) = (grid[0].len() as i32, grid.len() as i32);
    //     for i in 0..grid.len() {
    //         for j in 0..grid[0].len() {
    //             if (grid[i][j] == 1) {
    //                 len += dfs(i as i32, j as i32, &mut grid, x, y);
    //             }
    //         }
    //     }

    //     len
    //     // 12ms/2.5MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
    }
}
