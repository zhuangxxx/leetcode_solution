struct Solution;

impl Solution {
    /// [3232. 判断是否可以赢得数字游戏](https://leetcode.cn/problems/find-if-digit-game-can-be-won/)
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let (one, two) = nums.into_iter().fold((0, 0), |(one, two), num| {
            if num < 10 {
                (one + num, two)
            } else {
                (one, two + num)
            }
        });

        one != two
        // 0ms/2.25MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::can_alice_win(vec![1, 2, 3, 4, 10]));
    }

    #[test]
    fn test2() {
        assert!(Solution::can_alice_win(vec![1, 2, 3, 4, 5, 14]));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_alice_win(vec![5, 5, 5, 25]));
    }
}
