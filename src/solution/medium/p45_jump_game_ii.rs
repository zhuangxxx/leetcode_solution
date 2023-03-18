struct Solution;

impl Solution {
    /// [45. 跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/)
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }

        let (mut l, mut r, mut c) = (0, 0, 0);
        while r < nums.len() - 1 {
            let p = r;
            for i in l..=r {
                let n = i + nums[i] as usize;
                if n > r {
                    r = n;
                }
            }
            l = p + 1;
            c += 1;
        }

        c
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::jump(vec![1]), 0);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::jump(vec![1, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0]), 4);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::jump(vec![2, 1]), 1);
    }
}
