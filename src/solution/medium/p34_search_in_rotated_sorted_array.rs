struct Solution;

impl Solution {
    /// [33. 搜索旋转排序数组](https://leetcode.cn/problems/search-in-rotated-sorted-array/)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if (nums[0] > target) ^ (nums[0] > nums[m]) ^ (target > nums[m]) {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if l == r && nums[l] == target {
            l as i32
        } else {
            -1
        }
        // 0ms/2.2MB
    }

    // pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    //     let (mut l, mut r) = (0, nums.len() - 1);
    //     while l <= r {
    //         let m = l + (r - l) / 2;
    //         if nums[m] == target {
    //             return m as i32;
    //         }
    //         if nums[0] <= nums[m] {
    //             if nums[0] <= target && target < nums[m] {
    //                 r = m - 1;
    //             } else {
    //                 l = m + 1;
    //             }
    //         } else {
    //             if nums[m] < target && target <= nums[nums.len() - 1] {
    //                 l = m + 1;
    //             } else {
    //                 r = m - 1;
    //             }
    //         }
    //     }

    //     -1
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::search(vec![5, 1, 3], 2), -1);
    }
}
