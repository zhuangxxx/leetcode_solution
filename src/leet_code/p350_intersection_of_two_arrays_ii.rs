struct Solution;

impl Solution {
    /// [350. 两个数组的交集 II](https://leetcode.cn/problems/intersection-of-two-arrays-ii/)
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut is_vec, mut map) = (Vec::new(), std::collections::HashMap::new());

        for num in nums1 {
            *map.entry(num).or_insert(0) += 1;
        }

        for num in nums2 {
            *map.entry(num).or_insert(0) -= 1;
            if map[&num] >= 0 {
                is_vec.push(num);
            }
        }

        is_vec
        // 0ms/2.1MB
    }

    // pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //     let (mut nums1, mut nums2) = (nums1, nums2);
    //     nums1.sort();
    //     nums2.sort();

    //     let mut is_vec = Vec::new();
    //     let (mut i1, mut i2) = (0, 0);
    //     while i1 < nums1.len() && i2 < nums2.len() {
    //         if nums1[i1] == nums2[i2] {
    //             is_vec.push(nums2[i2]);
    //             i1 += 1;
    //             i2 += 1;
    //         } else if nums1[i1] < nums2[i2] {
    //             i1 += 1;
    //         } else {
    //             i2 += 1;
    //         }
    //     }

    //     is_vec
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
    }

    #[test]
    fn test2() {
        // Result order is random.
        let mut result = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort();
        assert_eq!(result, vec![4, 9]);
    }
}
