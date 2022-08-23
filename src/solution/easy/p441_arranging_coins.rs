struct Solution;

impl Solution {
    /// [441. 排列硬币](https://leetcode.cn/problems/arranging-coins/)
    pub fn arrange_coins(n: i32) -> i32 {
        (8. * n as f64 + 1.).sqrt() as i32 - 1 >> 1
        // 0ms/2MB
    }

    // pub fn arrange_coins(n: i32) -> i32 {
    //     let (mut l, mut r) = (1, n);
    //     while l < r {
    //         let m = (r - l + 1) / 2 + l;
    //         if m as i64 * (m as i64 + 1i64) <= 2i64 * n as i64 {
    //             l = m
    //         } else {
    //             r = m - 1
    //         }
    //     }

    //     l
    //     // 0ms/2MB
    // }

    // pub fn arrange_coins(n: i32) -> i32 {
    //     let (mut n, mut k) = (n, 0);

    //     while n - k > 0 {
    //         k += 1;
    //         n -= k;
    //     }

    //     k
    //     // 4ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::arrange_coins(5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::arrange_coins(8), 3);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::arrange_coins(10), 4);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::arrange_coins(1), 1);
    }

    #[test]
    fn trap3() {
        assert_eq!(Solution::arrange_coins(i32::MAX), 65535);
    }
}
