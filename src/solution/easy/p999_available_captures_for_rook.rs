struct Solution;

impl Solution {
    /// [999. 可以被一步捕获的棋子数](https://leetcode.cn/problems/available-captures-for-rook/)
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut num = 0;
        'outer: for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    for x in (0..i).rev() {
                        if board[x][j] == 'p' {
                            num += 1;
                            break;
                        } else if board[x][j] == 'B' {
                            break;
                        }
                    }
                    for x in i + 1..8 {
                        if board[x][j] == 'p' {
                            num += 1;
                            break;
                        } else if board[x][j] == 'B' {
                            break;
                        }
                    }
                    for y in (0..j).rev() {
                        if board[i][y] == 'p' {
                            num += 1;
                            break;
                        } else if board[i][y] == 'B' {
                            break;
                        }
                    }
                    for y in j + 1..8 {
                        if board[i][y] == 'p' {
                            num += 1;
                            break;
                        } else if board[i][y] == 'B' {
                            break;
                        }
                    }

                    break 'outer;
                }
            }
        }

        num
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
                vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            0
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::num_rook_captures(vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            3
        )
    }
}
