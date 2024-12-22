struct Solution;

impl Solution {
    /// [3264. K 次乘运算后的最终数组 I](https://leetcode.cn/problems/final-array-state-after-k-multiplication-operations-i/)
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::from_iter(
            nums.into_iter()
                .enumerate()
                .map(|(i, n)| std::cmp::Reverse((n, i))),
        );

        for _ in 0..k {
            if let Some(std::cmp::Reverse((n, i))) = heap.pop() {
                heap.push(std::cmp::Reverse((n * multiplier, i)));
            }
        }

        let mut nums = vec![0; heap.len()];
        while let Some(std::cmp::Reverse((n, i))) = heap.pop() {
            nums[i] = n;
        }

        nums
        // 0ms/2.28MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), vec![16, 8]);
    }
}
