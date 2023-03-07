struct Solution;

impl Solution {
    /// [31. 下一个排列](https://leetcode.cn/problems/next-permutation/)
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let len = nums.len();

        let mut ord = len;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                ord = i;
                break;
            }
        }

        if ord != len {
            for rev in (0..len).rev() {
                if nums[rev] > nums[ord] {
                    nums.swap(ord, rev);
                    break;
                }
            }
        }

        nums[if ord != len { ord + 1 } else { 0 }..].reverse();
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
