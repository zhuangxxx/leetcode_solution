struct Solution;

impl Solution {
    /// [50. Pow(x, n)](https://leetcode.cn/problems/powx-n/)
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn fast_pow(x: f64, mut n: i32) -> f64 {
            let (mut pow, mut c) = (1., x);
            while n > 0 {
                if n & 1 == 1 {
                    pow *= c;
                }
                c *= c;
                n >>= 1;
            }

            pow
        }

        if n.is_positive() {
            fast_pow(x, n)
        } else {
            1. / fast_pow(x, if n == i32::MIN { i32::MAX - 1 } else { -n })
        }
        // 0ms/2MB
    }

    // pub fn my_pow(mut x: f64, mut n: i32) -> f64 {
    //     if n < 0 {
    //         x = 1. / x;
    //         n = if n == i32::MIN { i32::MAX - 1 } else { -n };
    //     }

    //     let mut pow = 1.;
    //     for i in 0..31 {
    //         if n == 0 {
    //             break;
    //         }
    //         if n & 1 == 1 {
    //             let mut p = x;
    //             for _ in 0..i {
    //                 p *= p;
    //             }
    //             pow *= p;
    //         }
    //         n >>= 1;
    //     }

    //     pow
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!((Solution::my_pow(2., 10) - 1024.).abs() < 0.00001);
    }

    #[test]
    fn test2() {
        assert!((Solution::my_pow(2.1, 3) - 9.261).abs() < 0.00001);
    }

    #[test]
    fn test3() {
        assert!((Solution::my_pow(2., -2) - 0.25).abs() < 0.00001);
    }

    #[test]
    fn fail1() {
        assert!((Solution::my_pow(-1., i32::MIN) - 1.).abs() < 0.00001);
    }

    #[test]
    fn fail2() {
        assert!((Solution::my_pow(-1., i32::MAX) - -1.).abs() < 0.00001);
    }

    #[test]
    fn fail3() {
        assert!((Solution::my_pow(2., i32::MIN) - 0.).abs() < 0.00001);
    }
}
