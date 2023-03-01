struct Solution;

impl Solution {
    /// [18. 四数之和](https://leetcode.cn/problems/4sum/)
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut vec = Vec::new();
        for i in 0..nums.len() - 1 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in (i + 1..nums.len()).rev() {
                if j < nums.len() - 1 && nums[j] == nums[j + 1] {
                    continue;
                }
                let (mut l, mut r) = (i + 1, j - 1);
                while l < r {
                    let sum = {
                        let mut sum = nums[i];
                        if let Some(s) = sum.checked_add(nums[j]) {
                            sum = s;
                        } else {
                            break;
                        }
                        if let Some(s) = sum.checked_add(nums[l]) {
                            sum = s;
                        } else {
                            l += 1;
                            continue;
                        }
                        if let Some(s) = sum.checked_add(nums[r]) {
                            sum = s;
                        } else {
                            r -= 1;
                            continue;
                        }

                        sum
                    };
                    if sum < target {
                        l += 1;
                    } else if sum > target {
                        r -= 1;
                    } else {
                        vec.push(vec![nums[i], nums[l], nums[r], nums[j]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                }
            }
        }

        vec
        // 12ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            ),
            Vec::<Vec<i32>>::new()
        );
    }
}
