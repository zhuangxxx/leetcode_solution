struct Solution;

impl Solution {
    /// [495. 提莫攻击](https://leetcode.cn/problems/teemo-attacking/)
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut time = 0;

        for i in 0..time_series.len() - 1 {
            time += std::cmp::min(duration, time_series[i + 1] - time_series[i]);
        }

        time + duration
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }
}
