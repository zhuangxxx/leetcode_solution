struct Solution;

impl Solution {
    /// [367. 有效的完全平方数](https://leetcode.cn/problems/valid-perfect-square/)
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut l, mut r) = (0, num as i64);

        while l <= r {
            let m = l + (r - l) / 2;
            let square = m * m;
            if square > num as i64 {
                r = m - 1;
            } else if square < num as i64 {
                l = m + 1;
            } else {
                return true;
            }
        }

        false
        // 0ms/2.1MB
    }

    // pub fn is_perfect_square(num: i32) -> bool {
    //     let mut x0 = num as f32;

    //     loop {
    //         let x1 = (x0 + num as f32 / x0) / 2.;
    //         if (x0 - x1 < 1e-6) {
    //             break;
    //         }

    //         x0 = x1;
    //     }

    //     x0 as i32 * x0 as i32 == num
    //     // 0ms/2.1MB
    // }

    // pub fn is_perfect_square(num: i32) -> bool {
    //     let (mut sum, mut num) = (1, num);

    //     while (num > 0) {
    //         num -= sum;
    //         sum += 2;
    //         println!("{num}|{sum}");
    //     }

    //     num == 0
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_perfect_square(14));
    }

    #[test]
    fn trap1() {
        assert!(Solution::is_perfect_square(1));
    }

    #[test]
    fn trap2() {
        assert!(!Solution::is_perfect_square(i32::MAX));
    }
}
