struct Solution;

impl Solution {
    /// [29. 两数相除](https://leetcode.cn/problems/divide-two-integers/)
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if divisor == -1 {
            return if dividend == i32::MIN {
                i32::MAX
            } else {
                -dividend
            };
        } else if divisor == 1 {
            return dividend;
        }

        let is_neg = dividend.is_negative() ^ divisor.is_negative();
        if dividend.is_positive() {
            dividend = -dividend;
        }
        if divisor.is_positive() {
            divisor = -divisor;
        }

        let (mut div, mut min, mut curr) = (divisor, dividend >> 1, -1);
        while div >= min && div >= i32::MIN >> 1 {
            div <<= 1;
            curr <<= 1;
        }

        let mut quot = 0;
        while dividend <= divisor {
            while div < dividend {
                div >>= 1;
                curr >>= 1;
            }
            while dividend <= div {
                dividend -= div;
                quot -= curr;
            }
        }

        if quot == i32::MIN && !is_neg {
            i32::MAX
        } else if is_neg {
            -quot
        } else {
            quot
        }
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::divide(10, 3), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::divide(i32::MIN, 2), i32::MIN / 2);
    }
}
