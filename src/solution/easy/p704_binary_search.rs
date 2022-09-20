struct Solution;

impl Solution {
    /// [704. 二分查找](https://leetcode.cn/problems/binary-search/)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] < target {
                l = m + 1;
            } else if nums[m] > target {
                if let Some(m) = m.checked_sub(1) {
                    r = m;
                } else {
                    return -1;
                }
            } else {
                return m as i32;
            }
        }

        -1
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::search(vec![0], 1), -1);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::search(vec![1], 1), 0);
    }

    #[test]
    fn trap3() {
        assert_eq!(Solution::search(vec![-1, 1], 0), -1);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::search(vec![2, 5], 5), 1);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::search(vec![2, 5], 2), 0);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }
}
