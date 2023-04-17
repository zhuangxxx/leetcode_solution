struct Solution;

impl Solution {
    /// [79. 单词搜索](https://leetcode.cn/problems/word-search/)
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn check(
            board: &Vec<Vec<char>>,
            visit: &mut Vec<Vec<bool>>,
            index: (usize, usize),
            chars: &Vec<char>,
            len: usize,
        ) -> bool {
            const OPS: [(char, char); 4] = [('-', ' '), ('+', ' '), (' ', '-'), (' ', '+')];

            if len == chars.len() {
                return true;
            }

            visit[index.0][index.1] = true;
            for ops in OPS {
                let (i, j) = match ops {
                    ('-', ' ') => {
                        if index.0 == 0 {
                            continue;
                        }
                        (index.0 - 1, index.1)
                    }
                    ('+', ' ') => {
                        if index.0 == board.len() - 1 {
                            continue;
                        }
                        (index.0 + 1, index.1)
                    }
                    (' ', '-') => {
                        if index.1 == 0 {
                            continue;
                        }
                        (index.0, index.1 - 1)
                    }
                    (' ', '+') => {
                        if index.1 == board[0].len() - 1 {
                            continue;
                        }
                        (index.0, index.1 + 1)
                    }
                    _ => continue,
                };
                if !visit[i][j] && board[i][j] == chars[len] {
                    if check(board, visit, (i, j), chars, len + 1) {
                        return true;
                    }
                }
            }
            visit[index.0][index.1] = false;

            false
        }

        let mut visit = vec![vec![false; board[0].len()]; board.len()];
        let chars = word.chars().collect::<Vec<_>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == chars[0] && check(&board, &mut visit, (i, j), &chars, 1) {
                    return true;
                }
            }
        }

        false
        // 128ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            String::from("ABCCED")
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            String::from("SEE")
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            String::from("ABCB")
        ));
    }

    #[test]
    fn time1() {
        assert!(!Solution::exist(
            vec![
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N'],
                vec!['N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N', 'N']
            ],
            String::from("NNNNNNNNNNNNNB")
        ));
    }
}
