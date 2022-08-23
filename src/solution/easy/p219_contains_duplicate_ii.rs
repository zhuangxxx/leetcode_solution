struct Solution;

impl Solution {
    /// [219. 存在重复元素 II](https://leetcode.cn/problems/contains-duplicate-ii/)
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        let k = k as usize;

        for i in 0..nums.len() {
            if i > k {
                set.remove(&nums[i - k - 1]);
            }

            if !set.insert(nums[i]) {
                return true;
            }
        }

        false
        // 12ms/3.2MB
    }

    // pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    //     let mut map = std::collections::HashMap::new();
    //     let k = k as usize;

    //     for i in 0..nums.len() {
    //         if map.contains_key(&nums[i]) {
    //             if i - map.get(&nums[i]).unwrap() <= k {
    //                 return true;
    //             }
    //         }

    //         map.insert(nums[i], i);
    //     }

    //     false
    //     // 24ms/6.8MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn test2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn test3() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }
}
