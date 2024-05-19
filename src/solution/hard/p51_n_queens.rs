struct Solution;

impl Solution {
    /// [51. N 皇后](https://leetcode.cn/problems/n-queens/)
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        #[inline]
        fn check(chessboard: &[Vec<char>], row: usize, col: usize) -> bool {
            for i in 0..row {
                let sub = row - i;
                if chessboard[i][col] == 'Q'
                    || (col >= sub && chessboard[i][col - sub] == 'Q')
                    || (col < chessboard[0].len() - sub && chessboard[i][col + sub] == 'Q')
                {
                    return false;
                }
            }

            true
        }
        fn bt(chessboard: &mut Vec<Vec<char>>, row: usize) -> Vec<Vec<String>> {
            if row == chessboard.len() {
                return vec![chessboard
                    .clone()
                    .into_iter()
                    .map(|row| row.into_iter().collect())
                    .collect()];
            }

            let mut result = Vec::new();
            for j in 0..chessboard[row].len() {
                if check(chessboard, row, j) {
                    chessboard[row][j] = 'Q';
                    result.append(&mut bt(chessboard, row + 1));
                    chessboard[row][j] = '.';
                }
            }

            result
        }

        bt(&mut vec![vec!['.'; n as usize]; n as usize], 0)
        // 3ms/2.15MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
    }
}
