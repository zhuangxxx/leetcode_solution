struct Solution;

impl Solution {
    /// [153. 寻找旋转排序数组中的最小值](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/)
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] < nums[r] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        nums[l]
        // 0ms/2.01MB
    }

    // pub fn find_min(nums: Vec<i32>) -> i32 {
    //     if nums.len() == 1 || nums[0] < nums[nums.len() - 1] && nums[0] < nums[1] {
    //         return nums[0];
    //     } else if nums[nums.len() - 1] < nums[0] && nums[nums.len() - 1] < nums[nums.len() - 2] {
    //         return nums[nums.len() - 1];
    //     }

    //     let (mut l, mut r) = (0, nums.len() - 1);
    //     while l < r {
    //         let m = l + (r - l) / 2;
    //         if nums[m] < nums[if m == 0 { nums.len() - 1 } else { m - 1 }]
    //             && nums[m] < nums[if m == nums.len() - 1 { 0 } else { m + 1 }]
    //         {
    //             return nums[m];
    //         }
    //         if nums[m] > nums[l] && nums[m] > nums[r] {
    //             l = m;
    //         } else {
    //             r = m;
    //         }
    //     }

    //     nums[l]
    //     // 0ms/2.07MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::find_min(vec![1]), 1);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::find_min(vec![2, 3, 1]), 1);
    }
}
