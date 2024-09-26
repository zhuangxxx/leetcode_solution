struct Solution;

impl Solution {
    /// [2535. 数组元素和与数字和的绝对差](https://leetcode.cn/problems/difference-between-element-sum-and-digit-sum-of-an-array/)
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold([0, 0], |[mut s1, mut s2], mut n| {
                s1 += n;
                while n > 0 {
                    s2 += n % 10;
                    n /= 10;
                }
                [s1, s2]
            })
            .into_iter()
            .fold(0, |acc, x| if acc == 0 { x } else { acc - x })
        // 4ms/2.07MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::difference_of_sum(vec![1, 15, 6, 3]), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
