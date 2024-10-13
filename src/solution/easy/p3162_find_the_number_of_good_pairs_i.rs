struct Solution;

impl Solution {
    /// [3162. 优质数对的总数 I](https://leetcode.cn/problems/find-the-number-of-good-pairs-i/)
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut num = 0;

        let nums2 = nums2.into_iter().map(|n2| n2 * k).collect::<Vec<_>>();
        for n1 in nums1 {
            for n2 in nums2.iter() {
                num += if n1 % n2 == 0 { 1 } else { 0 };
            }
        }

        num
        // 1ms/2.04MB
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
