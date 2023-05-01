struct Solution;

impl Solution {
    /// [594. 最长和谐子序列](https://leetcode.cn/problems/longest-harmonious-subsequence/)
    pub fn find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort();

        let mut len = 0;
        let mut i = 0;
        for j in 0..nums.len() {
            while nums[j] - nums[i] > 1 {
                i += 1;
            }

            if nums[j] - nums[i] == 1 && j - i + 1 > len {
                len = j - i + 1;
            }
        }

        len as i32
        // 16ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::find_lhs(vec![1, 2, 2, 1]), 4);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::find_lhs(vec![1, 3, 5, 7, 9, 11, 13, 15, 17]), 0);
    }
}
