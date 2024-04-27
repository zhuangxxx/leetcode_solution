struct Solution;

impl Solution {
    /// [174. 地下城游戏](https://leetcode.cn/problems/dungeon-game/)
    pub fn calculate_minimum_hp(mut dungeon: Vec<Vec<i32>>) -> i32 {
        for i in (0..dungeon.len()).rev() {
            for j in (0..dungeon[i].len()).rev() {
                dungeon[i][j] += match (i.cmp(&(dungeon.len() - 1)), j.cmp(&(dungeon[i].len() - 1)))
                {
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => 0,
                    (std::cmp::Ordering::Equal, _) => dungeon[i][j + 1],
                    (_, std::cmp::Ordering::Equal) => dungeon[i + 1][j],
                    (_, _) => std::cmp::max(dungeon[i + 1][j], dungeon[i][j + 1]),
                };
                if dungeon[i][j] > 0 {
                    dungeon[i][j] = 0;
                }
            }
        }

        1 - dungeon[0][0]
        // 0ms/2.29MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ]),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
    }
}
