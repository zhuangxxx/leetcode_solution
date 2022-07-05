struct Solution;

impl Solution {
    /// [190. 颠倒二进制位](https://leetcode.cn/problems/reverse-bits/)
    pub fn reverse_bits(x: u32) -> u32 {
        let (m1, m2, m4, m8) = (0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff);
        let mut x = x;

        x = x >> 1 & m1 | (x & m1) << 1;
        x = x >> 2 & m2 | (x & m2) << 2;
        x = x >> 4 & m4 | (x & m4) << 4;
        x = x >> 8 & m8 | (x & m8) << 8;

        x >> 16 | x << 16
        // 0ms/2.2MB
    }

    // pub fn reverse_bits(x: u32) -> u32 {
    //     let (mut x, mut rev) = (x, 0);

    //     for i in 0..32 {
    //         rev |= (x & 1) << 31 - i;
    //         x >>= 1;

    //         if x == 0 {
    //             break;
    //         }
    //     }

    //     rev
    //     // 0ms/2MB
    // }

    // pub fn reverse_bits(x: u32) -> u32 {
    //     let mut rev = 0;

    //     for i in 0..32 {
    //         rev += ((x >> i) & 1) << (31 - i);
    //     }

    //     rev
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }
}
