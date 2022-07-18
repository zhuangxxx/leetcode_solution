struct Solution;

impl Solution {
    /// [349. 两个数组的交集](https://leetcode.cn/problems/intersection-of-two-arrays/)
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .intersection(&nums2.into_iter().collect::<std::collections::HashSet<_>>())
            .map(|&n| n)
            .collect()
        // 0ms/2.2MB
    }

    // pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //     let (mut nums1, mut nums2) = (nums1, nums2);
    //     nums1.sort();
    //     nums2.sort();

    //     let mut is_vec = Vec::new();
    //     let (mut i1, mut i2) = (0, 0);
    //     while i1 < nums1.len() && i2 < nums2.len() {
    //         if nums1[i1] == nums2[i2] {
    //             if is_vec.is_empty() || is_vec.last().unwrap() != &nums1[i1] {
    //                 is_vec.push(nums2[i2]);
    //             }
    //             i1 += 1;
    //             i2 += 1;
    //         } else if nums1[i1] < nums2[i2] {
    //             i1 += 1;
    //         } else {
    //             i2 += 1;
    //         }
    //     }

    //     is_vec
    //     // 0ms/2.2MB
    // }

    // pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //     let (mut set, mut is_set) = (
    //         std::collections::HashSet::new(),
    //         std::collections::HashSet::new(),
    //     );

    //     for num in nums1 {
    //         set.insert(num);
    //     }

    //     for num in nums2 {
    //         if set.contains(&num) {
    //             is_set.insert(num);
    //         }
    //     }

    //     is_set.iter().map(|&n| n).collect()
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }

    #[test]
    fn test2() {
        // Result order is random.
        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }

    #[test]
    fn fail1() {
        // Result order is random.
        let mut result =
            Solution::intersection(vec![4, 7, 9, 7, 6, 7], vec![5, 0, 0, 6, 1, 6, 2, 2, 4]);
        result.sort();
        assert_eq!(result, vec![4, 6]);
    }
}
