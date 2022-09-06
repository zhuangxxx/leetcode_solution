struct Solution;

impl Solution {
    // [643. 子数组最大平均数 I](https://leetcode.cn/problems/maximum-average-subarray-i/)
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = nums[0..k as usize].iter().sum::<i32>();
        let mut max = sum;
        for i in k as usize..nums.len() {
            sum = sum - nums[i - k as usize] + nums[i];
            max = std::cmp::max(max, sum);
        }

        max as f64 / k as f64
        // 16ms/2.8MB
    }

    // pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    //     let mut sum = i32::MIN;
    //     for i in 0..=nums.len() - k as usize {
    //         sum = std::cmp::max(sum, nums[i..i + k as usize].iter().sum());
    //     }

    //     sum as f64 / k as f64
    //     // 300ms/3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(
            (Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4) - 12.75).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn test2() {
        assert!((Solution::find_max_average(vec![5], 1) - 5.).abs() < f64::EPSILON);
    }

    #[test]
    fn fail1() {
        assert!((Solution::find_max_average(vec![0, 1, 1, 3, 3], 4) - 2.).abs() < f64::EPSILON);
    }

    #[test]
    fn fail2() {
        assert!((Solution::find_max_average(vec![0, 4, 0, 3, 2], 1) - 4.).abs() < f64::EPSILON);
    }
}
