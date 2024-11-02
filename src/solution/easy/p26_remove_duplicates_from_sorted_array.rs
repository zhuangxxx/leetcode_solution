struct Solution;

impl Solution {
    /// [26. 删除排序数组中的重复项](https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array/)
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();

        nums.len() as i32
        // 0ms/2.4MB
    }

    // pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    //     if nums.is_empty() {
    //         return 0;
    //     }

    //     let mut i = 0;
    //     let mut j = 1;
    //     while j < nums.len() {
    //         if nums[i] != nums[j] {
    //             i += 1;
    //             nums[i] = nums[j];
    //         }
    //         j += 1;
    //     }

    //     for _ in i + 1..nums.len() {
    //         nums.pop();
    //     }

    //     (i + 1) as i32
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(nums, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
