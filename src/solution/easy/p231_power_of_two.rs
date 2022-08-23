struct Solution;

impl Solution {
    /// [231. 2 的幂](https://leetcode.cn/problems/power-of-two/)
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & n - 1 == 0
        // 0ms/2.1MB
    }

    // pub fn is_power_of_two(n: i32) -> bool {
    //     n > 0 && n & -n == n
    //     // 0ms/2.1MB
    // }

    // pub fn is_power_of_two(n: i32) -> bool {
    //     n > 0 && (1 << 30) % n == 0
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_power_of_two(2));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_power_of_two(16));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_power_of_two(3));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_power_of_two(4));
    }

    #[test]
    fn test5() {
        assert!(!Solution::is_power_of_two(5));
    }
}
