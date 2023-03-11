struct Solution;

impl Solution {
    /// [36. 有效的数独](https://leetcode.cn/problems/valid-sudoku/)
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let (mut row, mut col, mut blk) = ([0; 9], [0; 9], [0; 9]);
        for y in 0..9 {
            for x in 0..9 {
                if board[y][x] == '.' {
                    continue;
                }
                let b = 1 << (board[y][x] as u8 - b'1') as usize;
                if row[x] & b != 0 || col[y] & b != 0 || blk[y / 3 * 3 + x / 3] & b != 0 {
                    return false;
                }
                row[x] |= b;
                col[y] |= b;
                blk[y / 3 * 3 + x / 3] |= b;
            }
        }

        true
        // 0ms/2.2MB
    }

    // pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //     let mut arr = [false; 9];
    //     for i in 0..3 {
    //         for j in 0..3 {
    //             for y in i * 3..i * 3 + 3 {
    //                 for x in j * 3..j * 3 + 3 {
    //                     if board[y][x] == '.' {
    //                         continue;
    //                     }
    //                     let u = (board[y][x] as u8 - b'1') as usize;
    //                     if arr[u] {
    //                         return false;
    //                     }
    //                     arr[u] = true;
    //                 }
    //             }
    //             arr.fill(false);
    //         }
    //     }

    //     for y in 0..9 {
    //         for x in 0..9 {
    //             if board[y][x] == '.' {
    //                 continue;
    //             }
    //             let u = (board[y][x] as u8 - b'1') as usize;
    //             if arr[u] {
    //                 return false;
    //             }
    //             arr[u] = true;
    //         }
    //         arr.fill(false);
    //     }

    //     for x in 0..9 {
    //         for y in 0..9 {
    //             if board[y][x] == '.' {
    //                 continue;
    //             }
    //             let u = (board[y][x] as u8 - b'1') as usize;
    //             if arr[u] {
    //                 return false;
    //             }
    //             arr[u] = true;
    //         }
    //         arr.fill(false);
    //     }

    //     true
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_valid_sudoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));
    }
}
