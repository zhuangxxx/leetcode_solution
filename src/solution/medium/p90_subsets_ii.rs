struct Solution;

impl Solution {
    /// [90. 子集 II](https://leetcode.cn/problems/subsets-ii/)
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut subsets = Vec::new();
        for mask in 0..1 << nums.len() {
            let mut subset = Vec::new();
            let mut duplicate = false;
            for i in 0..nums.len() {
                if mask & 1 << i == 0 {
                    continue;
                }
                if i > 0 && mask >> i - 1 & 1 == 0 && nums[i] == nums[i - 1] {
                    duplicate = true;
                    break;
                }
                subset.push(nums[i]);
            }
            if duplicate {
                continue;
            }
            subsets.push(subset);
        }

        subsets
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                Vec::new(),
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::subsets_with_dup(vec![0]),
            vec![Vec::new(), vec![0]]
        );
    }
}
