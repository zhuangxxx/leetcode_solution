struct Solution;

impl Solution {
    /// [342. 4的幂](https://leetcode.cn/problems/power-of-four/)
    pub fn is_power_of_four(n: i32) -> bool {
        // n > 0 && n & (n - 1) == 0 && n as u32 & 0xaaaaaaaau32 == 0
        n > 0 && n & (n - 1) == 0 && n % 3 == 1
        // 0ms/2MB
    }

    // pub fn is_power_of_four(n: i32) -> bool {
    //     let mut n = n;

    //     while n >= 4 {
    //         if n & 3 != 0 {
    //             break;
    //         }
    //         n /= 4;
    //     }

    //     n == 1
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_power_of_four(16));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_power_of_four(5));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_power_of_four(1));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_power_of_four(8));
    }
}
