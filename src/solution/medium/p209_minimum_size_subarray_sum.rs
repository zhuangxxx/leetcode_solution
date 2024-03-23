struct Solution;

impl Solution {
    /// [209. 长度最小的子数组](https://leetcode.cn/problems/minimum-size-subarray-sum/)
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut sum, mut min) = (0, usize::MAX);
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] >= target {
                return 1;
            }
            sum += nums[j];
            while sum >= target {
                min = std::cmp::min(min, j - i + 1);
                sum -= nums[i];
                i += 1;
            }
        }

        if min == usize::MAX {
            0
        } else {
            min as i32
        }
        // 4ms/2.98MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::min_sub_array_len(8, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            8
        )
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::min_sub_array_len(15, vec![2, 14]), 2)
    }
}
