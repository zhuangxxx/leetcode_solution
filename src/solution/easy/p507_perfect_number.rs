struct Solution;

impl Solution {
    /// [507. 完美数](https://leetcode.cn/problems/perfect-number/)
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 1;

        let mut i = 2;
        while i * i < num {
            if num % i == 0 {
                sum += i + num / i;
            }
            i += 1;
        }

        num != 1 && sum == num
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_perfect_number(28));
    }

    #[test]
    fn test2() {
        assert!(!Solution::check_perfect_number(7));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::check_perfect_number(1))
    }
}
