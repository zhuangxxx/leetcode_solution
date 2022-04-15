pub struct Solution;

impl Solution {
    /// [1. 两数之和](https://leetcode-cn.com/problems/two-sum/)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(j) = map.get(&complement) {
                return vec![*j as i32, i as i32];
            }
            map.insert(num, i);
        }
        
        Vec::<i32>::new()
        // 0ms/2.6MB
    }

    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     for i in 0..nums.len() {
    //         for j in i + 1..nums.len() {
    //             if nums[i] + nums[j] == target {
    //                 return vec![i as i32, j as i32];
    //             }
    //         }
    //     }
    //     Vec::<i32>::new()
    //     // 24ms/2.1MB
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
