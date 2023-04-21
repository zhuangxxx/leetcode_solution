struct Solution;

impl Solution {
    /// [81. 搜索旋转排序数组 II](https://leetcode.cn/problems/search-in-rotated-sorted-array-ii/)
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums[0] == target {
            return true;
        }

        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r && nums[l] == nums[r] {
            l += 1;
            r -= 1;
        }

        let (ln, rn) = (nums[l], nums[r]);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                l = m;
                break;
            }

            if target < ln {
                if target < nums[m] && nums[m] < ln {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                if target > nums[m] && nums[m] >= ln {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
        }

        nums[l] == target
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn test2() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn fail1() {
        assert!(Solution::search(
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 13, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            13
        ));
    }
}
