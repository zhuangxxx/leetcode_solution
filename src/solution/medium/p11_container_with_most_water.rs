struct Solution;

impl Solution {
    /// [11. 盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/)
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            if height[l] < height[r] {
                max = std::cmp::max(max, height[l] * (r - l) as i32);
                l += 1;
            } else {
                max = std::cmp::max(max, height[r] * (r - l) as i32);
                r -= 1;
            }
        }

        max
        // 8ms/2.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
