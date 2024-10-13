struct Solution;

impl Solution {
    /// [3158. 求出出现两次数字的 XOR 值](https://leetcode.cn/problems/find-the-xor-of-numbers-which-appear-twice/)
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut xor = 0;

        let mut map = [0; 51];
        for num in nums {
            if map[num as usize] > 0 {
                xor ^= num;
            } else {
                map[num as usize] += 1;
            }
        }

        xor
        // 2ms/2.03MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
    }
}
