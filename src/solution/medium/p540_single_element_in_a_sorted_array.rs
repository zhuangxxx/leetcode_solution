struct Solution;

impl Solution {
    /// [540. 有序数组中的单一元素](https://leetcode.cn/problems/single-element-in-a-sorted-array/)
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l < r {
            let m = l + (r - l) / 2;
            let m = m - (m & 1);
            if nums[m] == nums[m + 1] {
                l = m + 2;
            } else {
                r = m;
            }
        }

        nums[l]
        // 0ms/3.01MB
    }

    // pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    //     if nums.len() == 1 || nums[0] != nums[1] {
    //         return nums[0];
    //     }
    //     if nums[nums.len() - 2] != nums[nums.len() - 1] {
    //         return nums[nums.len() - 1];
    //     }

    //     let (mut l, mut r) = (0, nums.len() - 1);
    //     while l < r {
    //         let m = l + (r - l) / 2;
    //         if m & 1 == 0 {
    //             if nums[m] == nums[m + 1] {
    //                 l = m + 1;
    //             } else {
    //                 r = m;
    //             }
    //         } else {
    //             if nums[m - 1] == nums[m] {
    //                 l = m;
    //             } else {
    //                 r = m - 1;
    //             }
    //         }
    //         if r - l < 3 && nums[r] != nums[r - 1] && nums[r] != nums[r + 1] {
    //             return nums[r];
    //         }
    //     }

    //     nums[l]
    //     // 0ms/3.14MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
