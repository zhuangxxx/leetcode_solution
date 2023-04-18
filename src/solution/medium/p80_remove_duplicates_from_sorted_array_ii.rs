struct Solution;

impl Solution {
    /// [80. 删除有序数组中的重复项 II](https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/)
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut l, mut r, mut s) = (0, 0, 0);
        while r + s < nums.len() {
            if nums[l] != nums[r + s] {
                l = r;
                nums.swap(l, r + s);
                r += 1;
            } else if r - l < 2 {
                nums.swap(r, r + s);
                r += 1;
            } else {
                s += 1;
            }
        }

        (nums.len() - s) as i32
        // 0ms/2.2MB
    }

    // pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //     if nums.len() <= 2 {
    //         return nums.len() as i32;
    //     }

    //     let (mut f, mut s) = (2, 2);
    //     while f < nums.len() {
    //         if nums[s - 2] != nums[f] {
    //             nums[s] = nums[f];
    //             s += 1;
    //         }
    //         f += 1;
    //     }

    //     s as i32
    //     // 4ms/2.1MB
    // }

    // pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //     let (mut l, mut r) = (0, 0);
    //     while r < nums.len() {
    //         if nums[l] != nums[r] {
    //             l = r;
    //         } else if r - l < 2 {
    //             r += 1;
    //         } else {
    //             nums.remove(r);
    //         }
    //     }

    //     nums.len() as i32
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut result), 5);
        assert_eq!(result[0..5].to_vec(), vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn test2() {
        let mut result = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut result), 7);
        assert_eq!(result[0..7].to_vec(), vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
