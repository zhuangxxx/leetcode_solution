struct Solution;

impl Solution {
    /// [3238. 求出胜利玩家的数目](https://leetcode.cn/problems/find-the-number-of-winning-players/)
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut player = vec![[0; 11]; n as usize];
        for p in pick {
            player[p[0] as usize][p[1] as usize] += 1;
        }

        let mut cnt = 0;
        for (i, p) in player.into_iter().enumerate() {
            for c in p {
                if c > i {
                    cnt += 1;
                    break;
                }
            }
        }

        cnt
        // 1ms/2.24MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::winning_player_count(
                4,
                vec![
                    vec![0, 0],
                    vec![1, 0],
                    vec![1, 0],
                    vec![2, 1],
                    vec![2, 1],
                    vec![2, 0]
                ]
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
            1
        );
    }
}
