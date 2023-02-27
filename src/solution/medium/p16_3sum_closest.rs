struct Solution;

impl Solution {
    /// [16. 最接近的三数之和](https://leetcode.cn/problems/3sum-closest/)
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let m = nums[0..3].iter().sum();
        if m >= target {
            return m;
        }
        let x = nums[nums.len() - 3..nums.len()].iter().sum();
        if x <= target {
            return x;
        }

        let mut c = if target - m > x - target { m } else { x };
        for i in 0..nums.len() - 1 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let s = nums[i] + nums[l] + nums[r];
                if s < target {
                    if target - s < (target - c).abs() {
                        c = s;
                    }
                    l += 1;
                } else if s > target {
                    if s - target < (c - target).abs() {
                        c = s;
                    }
                    r -= 1;
                } else {
                    return target;
                }
            }
        }

        c
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
