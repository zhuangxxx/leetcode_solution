struct Solution;

impl Solution {
    /// [905. 按奇偶排序数组](https://leetcode.cn/problems/sort-array-by-parity/)
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by_key(|k| if k & 1 == 0 { 0 } else { 1 });

        nums
        // 4ms/2.1MB
    }

    // pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    //     let (mut l, mut r) = (0, nums.len() - 1);
    //     while l < r {
    //         while l < nums.len() - 1 && nums[l] & 1 == 0 {
    //             l += 1;
    //         }
    //         while r > 0 && nums[r] & 1 == 1 {
    //             r -= 1;
    //         }
    //         if l < r {
    //             nums.swap(l, r);
    //         }
    //     }

    //     nums
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![4, 2, 1, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::sort_array_by_parity(vec![0, 2]), vec![0, 2]);
    }
}
