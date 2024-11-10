struct Solution;

impl Solution {
    /// [3222. 求出硬币游戏的赢家](https://leetcode.cn/problems/find-the-winning-player-in-coin-game/)
    pub fn losing_player(x: i32, y: i32) -> String {
        if std::cmp::min(x, y / 4) & 1 == 1 {
            String::from("Alice")
        } else {
            String::from("Bob")
        }
        // 0ms/2.09MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::losing_player(2, 7), String::from("Alice"));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::losing_player(4, 11), String::from("Bob"));
    }
}
