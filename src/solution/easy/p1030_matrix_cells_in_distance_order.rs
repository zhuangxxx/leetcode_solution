struct Solution;

impl Solution {
    /// [1030. 距离顺序排列矩阵单元格](https://leetcode.cn/problems/matrix-cells-in-distance-order/)
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut cells = vec![
            Vec::new();
            (std::cmp::max(r_center, rows - r_center - 1)
                + std::cmp::max(c_center, cols - c_center - 1))
                as usize
                + 1
        ];
        for r in 0..rows {
            for c in 0..cols {
                cells[((r_center - r).abs() + (c_center - c).abs()) as usize].push(vec![r, c]);
            }
        }

        cells.into_iter().flatten().collect()
        // 8ms/3.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::all_cells_dist_order(1, 2, 0, 0),
            vec![vec![0, 0], vec![0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::all_cells_dist_order(2, 2, 0, 1),
            vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::all_cells_dist_order(2, 3, 1, 2),
            vec![
                vec![1, 2],
                vec![0, 2],
                vec![1, 1],
                vec![0, 1],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
