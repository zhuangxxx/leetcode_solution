struct Solution;

impl Solution {
    /// [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/)
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn binary_search(nums: &Vec<i32>, target: i32, lower: bool) -> usize {
            let (mut l, mut r, mut t) = (0, nums.len() - 1, nums.len());
            while l <= r {
                let m = l + (r - l) / 2;
                if nums[m] > target || (lower && nums[m] >= target) {
                    t = m;
                    if m == 0 {
                        break;
                    }
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            }

            t
        }

        if nums.is_empty() {
            return vec![-1, -1];
        }
        if nums[0] == nums[nums.len() - 1] {
            return if nums[0] == target {
                vec![0, nums.len() as i32 - 1]
            } else {
                vec![-1, -1]
            };
        }

        let l = binary_search(&nums, target, true);
        let r = binary_search(&nums, target, false) - 1;
        if l <= r && r < nums.len() && nums[l] == target && nums[r] == target {
            vec![l as i32, r as i32]
        } else {
            vec![-1, -1]
        }
        // 0ms/2.3MB
    }

    // pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     if nums.is_empty() {
    //         return vec![-1, -1];
    //     }
    //     if nums[0] == nums[nums.len() - 1] {
    //         return if nums[0] == target {
    //             vec![0, nums.len() as i32 - 1]
    //         } else {
    //             vec![-1, -1]
    //         };
    //     }

    //     let (mut l, mut r) = (0, nums.len() - 1);
    //     while l <= r {
    //         let m = l + (r - l) / 2;
    //         if nums[m] < target {
    //             l = m + 1;
    //         } else if nums[m] > target {
    //             r = m - 1;
    //         } else {
    //             l = m;
    //             while l > 0 && nums[l - 1] == target {
    //                 l -= 1;
    //             }
    //             r = m;
    //             while r < nums.len() - 1 && nums[r + 1] == target {
    //                 r += 1;
    //             }
    //             return vec![l as i32, r as i32];
    //         }
    //     }

    //     vec![-1, -1]
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::search_range(Vec::new(), 0), vec![-1, -1]);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::search_range(vec![1, 4], 4), vec![1, 1]);
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
    }

    #[test]
    fn fail3() {
        assert_eq!(Solution::search_range(vec![1, 3], 1), vec![0, 0]);
    }
}
