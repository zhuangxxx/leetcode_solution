struct Solution;

impl Solution {
    /// [191. 位1的个数](https://leetcode.cn/problems/number-of-1-bits/)
    pub fn hamming_weight(n: u32) -> i32 {
        let mut count = 0;

        for i in 0..32 {
            count += (n >> i) & 1;
        }

        count as i32
        // 0ms/1.9MB
    }

    // pub fn hamming_weight(n: u32) -> i32 {
    //     let (mut n, mut count) = (n, 0);

    //     while n > 0 {
    //         n &= n - 1;
    //         count += 1;
    //     }

    //     count
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::hamming_weight(0b00000000000000000000000000001011),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::hamming_weight(0b00000000000000000000000010000000),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::hamming_weight(0b11111111111111111111111111111101),
            31
        );
    }
}
