struct Solution;

impl Solution {
    /// [217. 存在重复元素](https://leetcode.cn/problems/contains-duplicate/)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();

        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                return true;
            }
        }

        false
        // 24ms/2.8MB
    }

    // pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    //     let mut map = std::collections::HashMap::new();

    //     for num in nums {
    //         if map.contains_key(&num) {
    //             return true;
    //         }

    //         map.insert(num, false);
    //     }

    //     false
    //     // 8ms/3.4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
