pub struct Solution;

impl Solution {
    /// [70. 爬楼梯](https://leetcode.cn/problems/climbing-stairs/)
    pub fn climb_stairs(n: i32) -> i32 {
        let mut step = n;
        let (mut prev1, mut prev2) = (1, 1);
        for _ in 1..n {
            step = prev1 + prev2;
            prev2 = prev1;
            prev1 = step;
        }

        step
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    fn trap1() {
        assert_eq!(Solution::climb_stairs(0), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }
}
