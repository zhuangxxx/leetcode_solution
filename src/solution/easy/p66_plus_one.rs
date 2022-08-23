struct Solution;

impl Solution {
    /// [66. 加一](https://leetcode.cn/problems/plus-one/)
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut len = digits.len();
        for i in (0..len).rev() {
            digits[i] = (digits[i] + 1) % 10;
            if digits[i] != 0 {
                return digits;
            }
        }

        digits = vec![0; len + 1];
        digits[0] = 1;
        digits
        // 0ms/2.2MB
    }

    // pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    //     let mut digits = digits;
    //     let mut i = digits.len() - 1;
    //     while i > 0 {
    //         if digits[i] != 9 {
    //             digits[i] += 1;
    //             return digits;
    //         }
    //         digits[i] = 0;
    //         i -= 1;
    //     }

    //     if digits[i] == 9 {
    //         digits[i] = 1;
    //         digits.push(0);
    //     } else {
    //         digits[i] += 1;
    //     }

    //     digits
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::plus_one(vec![7, 8, 9]), vec![7, 9, 0]);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
