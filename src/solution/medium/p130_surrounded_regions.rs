struct Solution;

impl Solution {
    /// [130. 被围绕的区域](https://leetcode.cn/problems/surrounded-regions/)
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn find(
            union_find: &mut Vec<Vec<(usize, usize)>>,
            target: (usize, usize),
        ) -> (usize, usize) {
            if union_find[target.0][target.1] != target {
                union_find[target.0][target.1] = find(union_find, union_find[target.0][target.1]);
            }
            union_find[target.0][target.1]
        }
        fn union(union_find: &mut Vec<Vec<(usize, usize)>>, p: (usize, usize), q: (usize, usize)) {
            let (i, j) = find(union_find, q);
            union_find[i][j] = find(union_find, p);
        }

        let (m, n) = (board.len(), board[0].len());
        let mut union_find: Vec<Vec<(usize, usize)>> =
            (0..=m).map(|i| (0..=n).map(|j| (i, j)).collect()).collect();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'X' {
                    continue;
                }
                if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                    union(&mut union_find, (i, j), (m, n));
                    continue;
                }
                for (x, y) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                    if board[x][y] == 'O' {
                        union(&mut union_find, (i, j), (x, y));
                    }
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'O'
                    && find(&mut union_find, (i, j)) != find(&mut union_find, (m, n))
                {
                    board[i][j] = 'X';
                }
            }
        }

        // 10ms/4.83MB
    }

    // pub fn solve(board: &mut Vec<Vec<char>>) {
    //     fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
    //         if board[i][j] != 'O' {
    //             return;
    //         }
    //         board[i][j] = 'A';
    //         if i > 0 {
    //             dfs(board, i - 1, j);
    //         }
    //         if i < board.len() - 1 {
    //             dfs(board, i + 1, j);
    //         }
    //         if j > 0 {
    //             dfs(board, i, j - 1);
    //         }
    //         if j < board[0].len() - 1 {
    //             dfs(board, i, j + 1);
    //         }
    //     }

    //     if board.is_empty() || board[0].is_empty() {
    //         return;
    //     }

    //     for i in 0..board.len() {
    //         dfs(board, i, 0);
    //         dfs(board, i, board[0].len() - 1);
    //     }
    //     for j in 1..board[0].len() - 1 {
    //         dfs(board, 0, j);
    //         dfs(board, board.len() - 1, j);
    //     }
    //     for i in 0..board.len() {
    //         for j in 0..board[0].len() {
    //             if board[i][j] == 'A' {
    //                 board[i][j] = 'O';
    //             } else if board[i][j] == 'O' {
    //                 board[i][j] = 'X';
    //             }
    //         }
    //     }
    //     // 8ms/4.82MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut result);
        assert_eq!(
            result,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X']
            ]
        );
    }

    #[test]
    fn test2() {
        let mut result = vec![vec!['X']];
        Solution::solve(&mut result);
        assert_eq!(result, vec![vec!['X']]);
    }

    #[test]
    fn fail1() {
        let mut result = vec![
            vec!['O', 'X', 'O', 'O', 'O', 'X'],
            vec!['O', 'O', 'X', 'X', 'X', 'O'],
            vec!['X', 'X', 'X', 'X', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'X', 'X'],
            vec!['X', 'X', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'X', 'X', 'X', 'X'],
        ];
        Solution::solve(&mut result);
        assert_eq!(
            result,
            vec![
                vec!['O', 'X', 'O', 'O', 'O', 'X'],
                vec!['O', 'O', 'X', 'X', 'X', 'O'],
                vec!['X', 'X', 'X', 'X', 'X', 'O'],
                vec!['O', 'O', 'O', 'O', 'X', 'X'],
                vec!['X', 'X', 'O', 'O', 'X', 'O'],
                vec!['O', 'O', 'X', 'X', 'X', 'X']
            ]
        )
    }
}
