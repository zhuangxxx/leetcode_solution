struct Solution;

impl Solution {
    /// [1009. 十进制整数的反码](https://leetcode.cn/problems/complement-of-base-10-integer/)
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            n ^ (1 << 32 - n.leading_zeros()) - 1
        }
        // 0ms/2MB
    }

    // pub fn bitwise_complement(n: i32) -> i32 {
    //     for b in 1..31 {
    //         if n >> b == 0 {
    //             return n ^ (1 << b) - 1;
    //         }
    //     }

    //     n ^ i32::MAX
    //     // 0ms/1.9MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::bitwise_complement(5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::bitwise_complement(7), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::bitwise_complement(10), 5);
    }
}
