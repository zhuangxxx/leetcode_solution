struct Solution;

impl Solution {
    /// [3248. 矩阵中的蛇](https://leetcode.cn/problems/snake-in-matrix/)
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut map = [("UP", -n), ("RIGHT", 1), ("DOWN", n), ("LEFT", -1)]
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>();

        commands
            .into_iter()
            .fold(0, |acc, command| acc + map[command.as_str()])
        // 0ms/2.23MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::final_position_of_snake(2, vec!["RIGHT".to_string(), "DOWN".to_string()]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::final_position_of_snake(
                3,
                vec!["DOWN".to_string(), "RIGHT".to_string(), "UP".to_string()]
            ),
            1
        );
    }
}
