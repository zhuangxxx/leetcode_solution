struct Solution;

impl Solution {
    /// [1035. 不相交的线](https://leetcode.cn/problems/uncrossed-lines/)
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                dp[i][j] = if nums1[i - 1] == nums2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    std::cmp::max(dp[i - 1][j], dp[i][j - 1])
                };
            }
        }

        dp[nums1.len()][nums2.len()]
        // 3ms/2.26MB
    }

    // pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    //     let mut dp = vec![vec![0; nums2.len()]; nums1.len()];
    //     dp[0][0] = if nums1[0] == nums2[0] { 1 } else { 0 };
    //     for i in 1..nums1.len() {
    //         dp[i][0] = std::cmp::max(dp[i - 1][0], if nums1[i] == nums2[0] { 1 } else { 0 });
    //     }
    //     for j in 1..nums2.len() {
    //         dp[0][j] = std::cmp::max(dp[0][j - 1], if nums1[0] == nums2[j] { 1 } else { 0 });
    //     }
    //     for i in 1..nums1.len() {
    //         for j in 1..nums2.len() {
    //             dp[i][j] = std::cmp::max(
    //                 dp[i - 1][j - 1] + if nums1[i] == nums2[j] { 1 } else { 0 },
    //                 std::cmp::max(dp[i - 1][j], dp[i][j - 1]),
    //             );
    //         }
    //     }

    //     dp[nums1.len() - 1][nums2.len() - 1]
    //     // 3ms/2.18MB
    // }
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::max_uncrossed_lines(vec![1], vec![1, 3]), 1);
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![3, 2], vec![2, 2, 2, 3]),
            1
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![3, 3], vec![1, 3, 1, 2, 1]),
            1
        );
    }
}
