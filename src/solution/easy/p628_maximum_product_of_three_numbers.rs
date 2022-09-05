struct Solution;

impl Solution {
    /// [628. 三个数的最大乘积](https://leetcode.cn/problems/maximum-product-of-three-numbers/)
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let (mut top3, mut bottom3) = ([i32::MIN; 3], [i32::MAX; 3]);
        for n in nums {
            if n > top3[2] {
                top3.swap(0, 1);
                top3.swap(1, 2);
                top3[2] = n;
            } else if n > top3[1] {
                top3.swap(0, 1);
                top3[1] = n;
            } else if n > top3[0] {
                top3[0] = n;
            }

            if n < bottom3[0] {
                bottom3.swap(1, 2);
                bottom3.swap(0, 1);
                bottom3[0] = n;
            } else if n < bottom3[1] {
                bottom3.swap(1, 2);
                bottom3[1] = n;
            } else if n < bottom3[2] {
                bottom3[2] = n;
            }
        }

        std::cmp::max(
            top3[0] * top3[1] * top3[2],
            bottom3[0] * bottom3[1] * top3[2],
        )
        // 4ms/2.3MB
    }

    // pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
    //     nums.sort();

    //     std::cmp::max(
    //         nums[0] * nums[1] * nums[nums.len() - 1],
    //         nums[nums.len() - 3] * nums[nums.len() - 2] * nums[nums.len() - 1],
    //     )
    //     // 12ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3]), -6);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::maximum_product(vec![-100, -98, -1, 2, 3, 4]),
            39200
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::maximum_product(vec![-1, -2, -3, -4]), -6);
    }
}
