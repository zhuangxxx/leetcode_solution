struct Solution;

impl Solution {
    /// [883. 三维形体投影面积](https://leetcode.cn/problems/projection-area-of-3d-shapes/)
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = (vec![0; grid[0].len()], vec![0; grid.len()], 0);
        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                if grid[j][i] > area.0[j] {
                    area.0[j] = grid[j][i];
                }
                if grid[j][i] > area.1[i] {
                    area.1[i] = grid[j][i];
                }
                if grid[j][i] > 0 {
                    area.2 += 1;
                }
            }
        }

        area.0.iter().sum::<i32>() + area.1.iter().sum::<i32>() + area.2
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
