struct Solution;

impl Solution {
    /// [55. 跳跃游戏](https://leetcode.cn/problems/jump-game/)
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32 - 1;

        let (mut l, mut r) = (0, 0);
        while l <= r {
            let p = r;
            for i in l..=r {
                r = std::cmp::max(r, i + nums[i as usize]);
            }
            if r >= n {
                break;
            }
            l = p + 1;
        }

        l <= r
        // 8ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
