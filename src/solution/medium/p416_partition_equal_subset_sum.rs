struct Solution;

impl Solution {
    /// [416. 分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/)
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let (mut sum, mut max) = (0, 0);
        for &num in nums.iter() {
            sum += num;
            max = std::cmp::max(max, num);
        }
        if sum & 1 == 1 {
            return false;
        }
        let target = sum >> 1;
        if max > target {
            return false;
        }

        let mut dp = vec![false; target as usize + 1];
        dp[0] = true;
        for num in nums {
            for j in (num as usize..=target as usize).rev() {
                dp[j] |= dp[j - num as usize];
            }
        }

        dp[target as usize]
        // 27ms/2.08MB
    }

    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let full = nums.iter().sum::<i32>();
    //     if full & 1 == 1 {
    //         return false;
    //     }

    //     let half = full as usize >> 1;
    //     let mut dp = vec![0; half + 1];
    //     for j in nums[0] as usize..=half {
    //         dp[j] = nums[0];
    //     }
    //     for i in 1..nums.len() {
    //         for j in (1..=half).rev() {
    //             if j as i32 >= nums[i] {
    //                 dp[j] = std::cmp::max(dp[j], dp[j - nums[i] as usize] + nums[i]);
    //             }
    //         }
    //     }

    //     dp[half] == half as i32
    //     // 50ms/2.18MB
    // }

    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let full = nums.iter().sum::<i32>();
    //     if full & 1 == 1 {
    //         return false;
    //     }

    //     let half = full as usize >> 1;
    //     let mut dp = vec![vec![0; half + 1]; nums.len()];
    //     for j in 0..=half {
    //         dp[0][j] = if j as i32 >= nums[0] { nums[0] } else { 0 };
    //     }
    //     for i in 1..nums.len() {
    //         for j in 1..=half {
    //             if j as i32 >= nums[i] {
    //                 dp[i][j] =
    //                     std::cmp::max(dp[i - 1][j], dp[i - 1][j - nums[i] as usize] + nums[i]);
    //             } else {
    //                 dp[i][j] = dp[i - 1][j];
    //             }
    //         }
    //     }

    //     dp[nums.len() - 1][half] == half as i32
    //     // 42ms/9.54MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::can_partition(vec![1, 2, 5]));
    }

    #[test]
    fn fail2() {
        assert!(!Solution::can_partition(vec![100]));
    }
}
