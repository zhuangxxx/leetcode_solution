struct Solution;

impl Solution {
    /// [283. 移动零](https://leetcode.cn/problems/move-zeroes/)
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        nums.retain(|&x| x != 0);
        nums.resize(len, 0);
        // 4ms/2.2MB
    }

    // pub fn move_zeroes(nums: &mut Vec<i32>) {
    //     let (mut slow, mut fast) = (0, 0);

    //     while fast < nums.len() {
    //         if nums[fast] != 0 {
    //             let val = nums[fast];
    //             nums[fast] = std::mem::replace(&mut nums[slow], val);
    //             slow += 1;
    //         }
    //         fast += 1;
    //     }
    //     // 8ms/2.2MB
    // }

    // pub fn move_zeroes(nums: &mut Vec<i32>) {
    //     for i in 0..nums.len() {
    //         if nums[i] != 0 {
    //             continue;
    //         }

    //         let mut j = i;
    //         while nums[i] == 0 && j < nums.len() {
    //             nums[i] = nums[j];
    //             nums[j] = 0;
    //             j += 1;
    //         }
    //     }
    //     // 72ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn fail1() {
        let mut nums = vec![0, 0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
