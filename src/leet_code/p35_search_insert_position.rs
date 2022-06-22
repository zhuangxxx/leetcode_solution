pub struct Solution;

impl Solution {
    /// [35. 搜索插入位置](https://leetcode.cn/problems/search-insert-position/)
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let middle = left + ((right - left) >> 1);
            if nums[middle] > target {
                right = middle;
            } else if nums[middle] < target {
                left = middle + 1;
            } else {
                return middle as i32;
            }
        }

        right as i32
        // 0ms/2.1MB
    }

    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     if nums.is_empty() || target < nums[0] {
    //         return 0;
    //     }

    //     if target > nums[nums.len() - 1] {
    //         return nums.len() as i32;
    //     }

    //     let mut i = nums.len() / 2;
    //     while i > 0 && i < nums.len() {
    //         if nums[i] > target {
    //             if nums[i - 1] < target {
    //                 return i as i32;
    //             }

    //             if nums[i - 1] == target {
    //                 return (i - 1) as i32;
    //             }

    //             i /= 2;
    //         } else if nums[i] < target {
    //             if i == nums.len() - 1 {
    //                 return i as i32 + 1;
    //             }

    //             if nums[i + 1] > target {
    //                 return (i + 1) as i32;
    //             }

    //             if nums[i + 1] == target {
    //                 return (i + 1) as i32;
    //             }

    //             i += i / 2;
    //         } else {
    //             return i as i32;
    //         }
    //     }

    //     if nums[i] >= target {
    //         i as i32
    //     } else {
    //         (i + 1) as i32
    //     }
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 4), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::search_insert(vec![1], 2), 1);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5], 5), 2);
    }

    #[test]
    fn fail4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5], 6), 3);
    }

    #[test]
    fn fail5() {
        assert_eq!(Solution::search_insert(vec![1, 4, 6, 7, 8, 9], 6), 2);
    }
}
