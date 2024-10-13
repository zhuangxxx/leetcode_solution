struct Solution;

impl Solution {
    /// [3164. 优质数对的总数 II](https://leetcode.cn/problems/find-the-number-of-good-pairs-ii/)
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut num = 0;

        let (mut map1, mut max1) = (std::collections::HashMap::new(), 0);
        for n1 in nums1.iter() {
            *map1.entry(n1).or_insert(0) += 1;
            max1 = std::cmp::max(max1, *n1);
        }

        let mut map2 = std::collections::HashMap::new();
        for n2 in nums2.iter() {
            *map2.entry(n2).or_insert(0) += 1;
        }

        for (&n2, cnt2) in map2 {
            let step = n2 * k;
            for n in (step..=max1).step_by(step as usize) {
                if let Some(&cnt1) = map1.get(&n) {
                    num += cnt1 * cnt2;
                }
            }
        }

        num
        // 91ms/8.28MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3),
            2
        );
    }
}
