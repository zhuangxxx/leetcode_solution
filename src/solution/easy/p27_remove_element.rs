struct Solution;

impl Solution {
    /// [27. 移除元素](https://leetcode-cn.com/problems/remove-element/)
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);

        nums.len() as i32
        // 0ms/2MB
    }

    // pub fn remove_element(nums: &mut Vec<i32>, val:i32) -> i32{
    //     if nums.is_empty() {
    //         return 0;
    //     }

    //     let (mut i, mut j) = (0, 0);

    //     while j < nums.len() {
    //         if nums[j] != val {
    //             nums[i] = nums[j];
    //             i += 1;
    //         }
    //         j += 1;
    //     }

    //     for _ in i..nums.len() {
    //         nums.pop();
    //     }

    //     nums.len() as i32
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(nums, vec![2, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(nums, vec![0, 1, 3, 0, 4]);
    }
}
