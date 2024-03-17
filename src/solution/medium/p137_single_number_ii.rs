struct Solution;

impl Solution {
    /// [137. 只出现一次的数字 II](https://leetcode.cn/problems/single-number-ii/)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (0, 0);
        for num in nums {
            let carry = low & num;
            low ^= num;
            high |= carry;
            let reset = low ^ high;
            low &= reset;
            high &= reset;
        }

        low
        // 1ms/2.23MB
    }

    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut n = 0;
    //     for i in 0..32 {
    //         let mut sum = 0;
    //         for &num in &nums {
    //             sum += num >> i & 1;
    //         }
    //         if sum % 3 != 0 {
    //             n |= 1 << i;
    //         }
    //     }

    //     n
    //     // 1ms/2.12MB
    // }

    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut map = std::collections::HashMap::new();
    //     for n in nums {
    //         *map.entry(n).or_insert(0) += 1;
    //     }
    //     for (k, v) in map {
    //         if v == 1 {
    //             return k;
    //         }
    //     }

    //     -1
    //     // 1ms/2.11MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
