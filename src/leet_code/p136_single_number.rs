struct Solution;

impl Solution {
    /// [136. 只出现一次的数字](https://leetcode.cn/problems/single-number/)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single = 0;

        for num in nums {
            single ^= num;
        }

        single
        // 0ms/2.2MB
    }

    // pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let mut map = std::collections::HashMap::new();

    //     for num in nums {
    //         if map.contains_key(&num) {
    //             map.insert(num, 2);
    //         } else {
    //             map.insert(num, 1);
    //         }
    //     }

    //     for (key, val) in map {
    //         if val == 1 {
    //             return key;
    //         }
    //     }

    //     0
    //     // 4ms/2.4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
