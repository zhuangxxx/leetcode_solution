struct Solution;

impl Solution {
    /// [717. 1 比特与 2 比特字符](https://leetcode.cn/problems/1-bit-and-2-bit-characters/)
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = bits.len() as i32 - 2;
        while i >= 0 && bits[i as usize] == 1 {
            i -= 1;
        }

        (bits.len() as i32 - i) & 1 == 0
        // 0ms/1.9MB
    }

    // pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    //     let mut i = 0;
    //     while i < bits.len() - 1 {
    //         i += bits[i] as usize + 1;
    //     }

    //     i == bits.len() - 1
    //     // 0ms/2.2MB
    // }

    // pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    //     if bits.len() <= 2 {
    //         return bits[0] == 0;
    //     }

    //     let mut i = 0;
    //     while i < bits.len() {
    //         if i == bits.len() - 3 && bits[i] == 1 || i == bits.len() - 2 && bits[i] == 0 {
    //             return true;
    //         } else if bits[i] == 1 {
    //             i += 2;
    //         } else if bits[i] == 0 {
    //             i += 1;
    //         }
    //     }

    //     false
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_one_bit_character(vec![1, 0, 0]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_one_bit_character(vec![1, 1, 1, 0]));
    }

    #[test]
    fn trap1() {
        assert!(Solution::is_one_bit_character(vec![0]));
    }

    #[test]
    fn trap2() {
        assert!(!Solution::is_one_bit_character(vec![1, 0]));
    }
}
