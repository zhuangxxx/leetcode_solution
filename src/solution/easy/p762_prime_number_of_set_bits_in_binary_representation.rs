struct Solution;

impl Solution {
    /// [762. 二进制表示中质数个计算置位](https://leetcode.cn/problems/prime-number-of-set-bits-in-binary-representation/)
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const MASK: i32 = 0b10100010100010101100;

        let mut count = 0;
        for mut n in left..=right {
            let mut c = 0;
            while n > 0 {
                n &= n - 1;
                c += 1;
            }

            if 1 << c & MASK > 0 {
                count += 1;
            }
        }

        count
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
    }
}
