struct Solution;

impl Solution {
    /// [633. 平方数之和](https://leetcode.cn/problems/sum-of-square-numbers/)
    pub fn judge_square_sum(mut c: i32) -> bool {
        for base in 2..=(c as f64).sqrt() as i32 {
            if c % base != 0 {
                continue;
            }

            let mut exp = 0;
            while c % base == 0 {
                c /= base;
                exp += 1;
            }

            if base & 3 == 3 && exp & 1 != 0 {
                return false;
            }
        }

        c & 3 != 3
        // 1ms/2.09MB
    }

    // pub fn judge_square_sum(c: i32) -> bool {
    //     let (mut l, mut r) = (0i64, (c as f64).sqrt() as i64);
    //     while l <= r {
    //         match (c as i64).cmp(&(l * l + r * r)) {
    //             std::cmp::Ordering::Less => r -= 1,
    //             std::cmp::Ordering::Greater => l += 1,
    //             _ => return true,
    //         }
    //     }

    //     false
    //     // 0ms/2.16MB
    // }

    // pub fn judge_square_sum(c: i32) -> bool {
    //     for a in 0..=(c as f32).sqrt().floor() as i32 {
    //         let b2 = c - a * a;
    //         let b = (b2 as f32).sqrt().floor() as i32;
    //         if b * b == b2 {
    //             return true;
    //         }
    //     }

    //     false
    //     // 2ms/2.57MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::judge_square_sum(5));
    }

    #[test]
    fn test2() {
        assert!(!Solution::judge_square_sum(3));
    }
}
