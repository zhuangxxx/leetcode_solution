struct Solution;

impl Solution {
    /// [747. 至少是其他数字两倍的最大数](https://leetcode.cn/problems/largest-number-at-least-twice-of-others/)
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let (mut index, mut first, mut second) = (0, 0, 0);
        for i in 0..nums.len() {
            if nums[i] > first {
                second = first;
                index = i;
                first = nums[index];
            } else if nums[i] < first && nums[i] > second {
                second = nums[i];
            }
        }

        if first >= second * 2 {
            index as i32
        } else {
            -1
        }
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::dominant_index(vec![1]), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::dominant_index(vec![0, 0, 3, 2]), -1);
    }
}
