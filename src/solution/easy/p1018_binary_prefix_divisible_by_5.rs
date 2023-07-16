struct Solution;

impl Solution {
    /// [1018. 可被 5 整除的二进制前缀](https://leetcode.cn/problems/binary-prefix-divisible-by-5/)
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        nums.into_iter()
            .scan(0, |rem, n| {
                *rem = ((*rem << 1) + n) % 5;
                Some(*rem)
            })
            .map(|rem| rem == 0)
            .collect()
        // 4ms/2.1MB
    }

    // pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    //     let mut dives = Vec::with_capacity(nums.len());
    //     let mut rem = 0;
    //     for num in nums {
    //         rem = ((rem << 1) + num) % 5;
    //         dives.push(rem == 0);
    //     }

    //     dives
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![
                1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                1, 1, 1, 1, 1
            ]),
            vec![
                false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, false, false, false, true, false, false, false, true,
                false, false, false, true, false, false, false, true, false
            ]
        );
    }
}
