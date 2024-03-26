struct Solution;

impl Solution {
    /// [454. 四数相加 II](https://leetcode.cn/problems/4sum-ii/)
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut count = 0;

        let mut map = std::collections::HashMap::new();
        for n1 in nums1 {
            for &n2 in &nums2 {
                *map.entry(n1 + n2).or_insert(0) += 1;
            }
        }
        for n3 in nums3 {
            for &n4 in &nums4 {
                if let Some(&c) = map.get(&(-n3 - n4)) {
                    count += c;
                }
            }
        }

        count
        // 54ms/2.09MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
            1
        );
    }
}
