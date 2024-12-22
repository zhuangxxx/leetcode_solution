struct Solution;

impl Solution {
    /// [2717. 半有序排列](https://leetcode.cn/problems/semi-ordered-permutation/)
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let (mut l, mut r) = (i32::MIN, i32::MAX);
        for (i, n) in nums.into_iter().enumerate() {
            match n {
                1 => l = i as i32,
                n if n == len => r = i as i32,
                _ => {
                    if l > 0 && r < len {
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }

        l - 1 + len - r - if l > r { 1 } else { 0 }
        // 0ms/2.29MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
