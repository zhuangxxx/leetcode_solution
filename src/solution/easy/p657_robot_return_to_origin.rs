struct Solution;

impl Solution {
    /// [657. 机器人能否返回原点](https://leetcode.cn/problems/robot-return-to-origin/)
    pub fn judge_circle(moves: String) -> bool {
        let mut pos = [0, 0];

        for m in moves.chars() {
            match m {
                'R' => pos[0] += 1,
                'L' => pos[0] -= 1,
                'U' => pos[1] += 1,
                'D' => pos[1] -= 1,
                _ => continue,
            }
        }

        pos == [0, 0]
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::judge_circle(String::from("UD")));
    }

    #[test]
    fn test2() {
        assert!(!Solution::judge_circle(String::from("LL")));
    }
}
