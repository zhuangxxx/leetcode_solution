struct Solution;

impl Solution {
    /// [461. 汉明距离](https://leetcode.cn/problems/hamming-distance/)
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
        // 0ms/2MB
    }

    // pub fn hamming_distance(x: i32, y: i32) -> i32 {
    //     let (mut n, mut s) = (x ^ y, 0);

    //     while n > 0 {
    //         s += 1;
    //         n &= n - 1;
    //     }

    //     s
    //     // 0ms/2MB
    // }

    // pub fn hamming_distance(x: i32, y: i32) -> i32 {
    //     let mut n = x ^ y;

    //     n = (n & 0x55555555) + (n >> 1 & 0x55555555);
    //     n = (n & 0x33333333) + (n >> 2 & 0x33333333);
    //     n = (n & 0x0f0f0f0f) + (n >> 4 & 0x0f0f0f0f);
    //     n = (n & 0x00ff00ff) + (n >> 8 & 0x00ff00ff);
    //     n = (n & 0x0000ffff) + (n >> 16 & 0x0000ffff);

    //     n
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
