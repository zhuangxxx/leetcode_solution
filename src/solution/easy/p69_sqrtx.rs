struct Solution;

impl Solution {
    /// [69. x 的平方根](https://leetcode.cn/problems/sqrtx/)
    pub fn my_sqrt(x: i32) -> i32 {
        let mut s = 0;

        let (mut left, mut right) = (1, x);
        while left <= right {
            let mid = left + (right - left) / 2;
            if mid <= x / mid {
                left = mid + 1;
                s = mid;
            } else {
                right = mid - 1;
            }
        }

        s
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::my_sqrt(10), 3);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn fail4() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn fail5() {
        assert_eq!(Solution::my_sqrt(2147483647), 46340);
    }

    #[test]
    fn fail6() {
        assert_eq!(Solution::my_sqrt(183692038), 13553);
    }
}
