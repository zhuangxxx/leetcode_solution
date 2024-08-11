struct Solution;

impl Solution {
    /// [494. 目标和](https://leetcode.cn/problems/target-sum/)
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let size = nums.iter().sum::<i32>() + target;
        if size.is_negative() || size & 1 == 1 {
            return 0;
        }
        let size = size as usize >> 1;

        let mut dp = vec![0; size + 1];
        dp[0] = 1;
        for num in nums {
            for j in (num as usize..=size).rev() {
                dp[j] += dp[j - num as usize];
            }
        }

        dp[size]
        // 3ms/2.14MB
    }

    // pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    //     fn bt(
    //         nums: &[i32],
    //         target: i32,
    //         pos: usize,
    //         sum: i32,
    //         mem: &mut std::collections::HashMap<(usize, i32), i32>,
    //     ) -> i32 {
    //         if pos == nums.len() {
    //             return if sum == target { 1 } else { 0 };
    //         }
    //         if let Some(&val) = mem.get(&(pos, sum)) {
    //             return val;
    //         }

    //         let val = bt(nums, target, pos + 1, sum + nums[pos], mem)
    //             + bt(nums, target, pos + 1, sum - nums[pos], mem);
    //         mem.insert((pos, sum), val);

    //         val
    //     }

    //     bt(&nums, target, 0, 0, &mut std::collections::HashMap::new())
    //     // 42ms/2.46MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
