struct Solution;

impl Solution {
    /// [52. N 皇后 II](https://leetcode.cn/problems/n-queens-ii/)
    pub fn total_n_queens(n: i32) -> i32 {
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
        fn bt(chessboard: &mut Vec<Vec<char>>, row: usize) -> i32 {
            if row == chessboard.len() {
                return 1;
            }

            let mut num = 0;
            for j in 0..chessboard[row].len() {
                if check(chessboard, row, j) {
                    chessboard[row][j] = 'Q';
                    num += bt(chessboard, row + 1);
                    chessboard[row][j] = '.';
                }
            }

            num
        }

        bt(&mut vec![vec!['.'; n as usize]; n as usize], 0)
        // 4ms/2.00MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
