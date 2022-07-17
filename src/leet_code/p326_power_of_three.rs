struct Solution;

impl Solution {
    /// [326. 3 的幂](https://leetcode.cn/problems/power-of-three/)
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 3i32.pow(19) % n == 0
        // 8ms/2MB
    }

    // pub fn is_power_of_three(n: i32) -> bool {
    //     let mut n = n;

    //     while n >= 3 {
    //         if n % 3 != 0 {
    //             break;
    //         }
    //         n /= 3;
    //     }

    //     n == 1
    //     // 12ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_power_of_three(27));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_power_of_three(0));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_power_of_three(9));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_power_of_three(45));
    }
}
