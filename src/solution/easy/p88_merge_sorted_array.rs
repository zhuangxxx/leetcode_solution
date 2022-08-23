struct Solution;

impl Solution {
    /// [88. 合并两个有序数组](https://leetcode.cn/problems/merge-sorted-array/)
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m + n - 1;
        let (mut m, mut n) = (m - 1, n - 1);

        while n >= 0 {
            while m >= 0 && nums1[m as usize] > nums2[n as usize] {
                nums1.swap(i as usize, m as usize);
                i -= 1;
                m -= 1;
            }

            std::mem::swap(&mut nums1[i as usize], &mut nums2[n as usize]);
            i -= 1;
            n -= 1;
        }
        // 0ms/2MB
    }

    // pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    //     let (mut i, mut j) = (0, 0);

    //     while i < m as usize + j && j < n as usize {
    //         if nums1[i] > nums2[j] {
    //             for k in i + 1..=m as usize + j {
    //                 nums1.swap(i, k);
    //             }
    //             std::mem::swap(&mut nums1[i], &mut nums2[j]);
    //             j += 1;
    //         }
    //         i += 1;
    //     }

    //     if i > m as usize {
    //         i = m as usize;
    //     }

    //     while j < n as usize {
    //         std::mem::swap(&mut nums1[i + j], &mut nums2[j]);
    //         j += 1;
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test2() {
        let (mut nums1, mut nums2) = (vec![1], vec![]);
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test3() {
        let (mut nums1, mut nums2) = (vec![0], vec![1]);
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn trap1() {
        let (mut nums1, mut nums2) = (vec![-1, 1, 0, 0, 0], vec![-2, 0, 2]);
        Solution::merge(&mut nums1, 2, &mut nums2, 3);
        assert_eq!(nums1, vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn trap2() {
        let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0], vec![8, 9]);
        Solution::merge(&mut nums1, 3, &mut nums2, 2);
        assert_eq!(nums1, vec![1, 2, 3, 8, 9]);
    }

    #[test]
    fn trap3() {
        let (mut nums1, mut nums2) = (vec![7, 8, 9, 0, 0], vec![1, 2]);
        Solution::merge(&mut nums1, 3, &mut nums2, 2);
        assert_eq!(nums1, vec![1, 2, 7, 8, 9]);
    }

    #[test]
    fn fail1() {
        let (mut nums1, mut nums2) = (vec![0, 0, 3, 0, 0, 0, 0, 0, 0], vec![-1, 1, 1, 1, 2, 3]);
        Solution::merge(&mut nums1, 3, &mut nums2, 6);
        assert_eq!(nums1, vec![-1, 0, 0, 1, 1, 1, 2, 3, 3]);
    }
}
