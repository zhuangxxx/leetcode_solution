struct Solution;

impl Solution {
    /// [3226. 使两个整数相等的位更改次数](https://leetcode.cn/problems/number-of-bit-changes-to-make-two-integers-equal/)
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k == k {
            (n ^ k).count_ones() as i32
        } else {
            -1
        }
        // 0ms/2.00MB
    }

    // pub fn min_changes(mut n: i32, mut k: i32) -> i32 {
    //     let mut x = 0;
    //     while n > 0 || k > 0 {
    //         if n & 1 != k & 1 {
    //             if k & 1 == 1 {
    //                 return -1;
    //             }
    //             x += 1;
    //         }
    //         n >>= 1;
    //         k >>= 1;
    //     }

    //     x
    //     // 0ms/2.01MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_changes(13, 4), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_changes(21, 21), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_changes(14, 13), -1);
    }
}
