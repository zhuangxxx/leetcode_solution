struct Solution;

impl Solution {
    /// [75. 颜色分类](https://leetcode.cn/problems/sort-colors/)
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut r, mut b) = (0, nums.len() - 1);
        let mut i = 0;
        while b > 0 && i <= b {
            if nums[i] == 0 {
                nums.swap(i, r);
                r += 1;
                i += 1;
            } else if nums[i] == 1 {
                i += 1;
            } else {
                nums.swap(i, b);
                b -= 1;
            }
        }
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test2() {
        let mut result = vec![2, 0, 1];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn trap1() {
        let mut result = vec![2, 2];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn trap2() {
        let mut result = vec![2];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn fail1() {
        let mut result = vec![1, 0, 2];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn fail2() {
        let mut result = vec![1, 2, 0];
        Solution::sort_colors(&mut result);
        assert_eq!(result, vec![0, 1, 2]);
    }
}
