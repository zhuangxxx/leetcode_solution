struct Solution;

/// [3242. 设计相邻元素求和服务](https://leetcode.cn/problems/design-neighbor-sum-service/)
struct NeighborSum {
    grid: Vec<Vec<i32>>,
    position: std::collections::HashMap<i32, (i32, i32)>,
    // 6ms/2.30MB
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut position = std::collections::HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                position.insert(grid[i][j], (i as i32, j as i32));
            }
        }

        NeighborSum { grid, position }
    }

    fn sum(&self, value: i32, range: [(i32, i32); 4]) -> i32 {
        let mut sum = 0;
        let (i, j) = self.position[&value];
        for (x, y) in range {
            let (i, j) = (i + x, j + y);
            if i < 0 || j < 0 || i >= self.grid.len() as i32 || j >= self.grid[0].len() as i32 {
                continue;
            }
            sum += self.grid[i as usize][j as usize];
        }

        sum
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        self.sum(value, [(-1, 0), (0, -1), (1, 0), (0, 1)])
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        self.sum(value, [(-1, -1), (-1, 1), (1, -1), (1, 1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut neighbor_sum = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(neighbor_sum.adjacent_sum(1), 6);
        assert_eq!(neighbor_sum.adjacent_sum(4), 16);
        assert_eq!(neighbor_sum.diagonal_sum(4), 16);
        assert_eq!(neighbor_sum.diagonal_sum(8), 4);
    }

    #[test]
    fn test2() {
        let mut neighbor_sum = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(neighbor_sum.adjacent_sum(15), 23);
        assert_eq!(neighbor_sum.diagonal_sum(9), 45);
    }
}
