struct Solution;

impl Solution {
    /// [922. 按奇偶排序数组 II](https://leetcode.cn/problems/sort-array-by-parity-ii/)
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .filter(|&x| x & 1 == 0)
            .zip(nums.iter().filter(|&x| x & 1 != 0))
            .map(|(&e, &o)| vec![e, o])
            .flatten()
            .collect()
        // 8ms/2.2MB
    }

    // pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
    //     let (mut e, mut o) = (0, 1);
    //     loop {
    //         while e <= nums.len() - 2 && nums[e] & 1 == 0 {
    //             e += 2;
    //         }
    //         while o <= nums.len() - 1 && nums[o] & 1 == 1 {
    //             o += 2;
    //         }
    //         if e > nums.len() - 2 || o > nums.len() - 1 {
    //             break;
    //         }
    //         nums.swap(e, o);
    //         e += 2;
    //         o += 2;
    //     }

    //     nums
    //     // 8ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }
}
