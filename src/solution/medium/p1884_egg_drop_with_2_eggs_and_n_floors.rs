struct Solution;

impl Solution {
    /// [1884. 鸡蛋掉落-两枚鸡蛋](https://leetcode.cn/problems/egg-drop-with-2-eggs-and-n-floors/)
    pub fn two_egg_drop(n: i32) -> i32 {
        let mut dp = vec![i32::MAX >> 1; n as usize + 1];
        dp[0] = 0;
        for i in 1..=n as usize {
            for j in 1..=i {
                dp[i] = std::cmp::min(dp[i], std::cmp::max(j as i32 - 1, dp[i - j]) + 1);
            }
        }

        dp[n as usize]
        // 22ms/1.91MB
    }

    // pub fn two_egg_drop(n: i32) -> i32 {
    //     let (mut x, mut s) = (1, 1);
    //     loop {
    //         s += x;
    //         if s > n {
    //             break;
    //         }
    //         x += 1;
    //     }

    //     x
    //     // 2ms/2.04MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_egg_drop(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_egg_drop(100), 14);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::two_egg_drop(6), 3);
    }
}
