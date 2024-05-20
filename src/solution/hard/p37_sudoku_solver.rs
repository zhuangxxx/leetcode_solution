struct Solution;

impl Solution {
    /// [37. 解数独](https://leetcode.cn/problems/sudoku-solver/)
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn bt(
            board: &mut Vec<Vec<char>>,
            col: &mut [[bool; 10]],
            row: &mut [[bool; 10]],
            grid: &mut [[bool; 10]],
            mut i: usize,
            mut j: usize,
        ) -> bool {
            if j == 9 {
                i += 1;
                j = 0;
            }
            if i == 9 {
                return true;
            }

            let k = i / 3 * 3 + j / 3 % 3;
            if board[i][j] != '.' {
                let n = (board[i][j] as u8 - b'0') as usize;
                col[j][n] = true;
                row[i][n] = true;
                grid[k][n] = true;
                return bt(board, col, row, grid, i, j + 1);
            }
            for n in 1..=9 {
                if row[i][n] || col[j][n] || grid[k][n] {
                    continue;
                }
                row[i][n] = true;
                col[j][n] = true;
                grid[k][n] = true;
                board[i][j] = (n as u8 + b'0') as char;
                if bt(board, col, row, grid, i, j + 1) {
                    return true;
                }
                board[i][j] = '.';
                grid[k][n] = false;
                col[j][n] = false;
                row[i][n] = false;
            }

            false
        }

        let (mut row, mut col, mut grid) = ([[false; 10]; 9], [[false; 10]; 9], [[false; 10]; 9]);
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let n = (board[i][j] as u8 - b'0') as usize;
                    row[i][n] = true;
                    col[j][n] = true;
                    grid[i / 3 * 3 + j / 3 % 3][n] = true;
                }
            }
        }

        bt(board, &mut col, &mut row, &mut grid, 0, 0);
        // 0ms/2.12MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }
}
