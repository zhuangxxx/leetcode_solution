struct Solution;

impl Solution {
    /// [892. 三维形体的表面积](https://leetcode.cn/problems/surface-area-of-3d-shapes/)
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;
        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                if grid[j][i] == 0 {
                    continue;
                }
                area += 2;
                if j == 0 {
                    area += grid[j][i];
                } else if grid[j][i] > grid[j - 1][i] {
                    area += grid[j][i] - grid[j - 1][i];
                }
                if j == grid.len() - 1 {
                    area += grid[j][i];
                } else if grid[j][i] > grid[j + 1][i] {
                    area += grid[j][i] - grid[j + 1][i];
                }
                if i == 0 {
                    area += grid[j][i]
                } else if grid[j][i] > grid[j][i - 1] {
                    area += grid[j][i] - grid[j][i - 1];
                }
                if i == grid[0].len() - 1 {
                    area += grid[j][i]
                } else if grid[j][i] > grid[j][i + 1] {
                    area += grid[j][i] - grid[j][i + 1];
                }
            }
        }

        area
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            32
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
            46
        );
    }
}
