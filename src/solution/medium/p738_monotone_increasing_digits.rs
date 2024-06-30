struct Solution;

impl Solution {
    /// [738. 单调递增的数字](https://leetcode.cn/problems/monotone-increasing-digits/)
    pub fn monotone_increasing_digits(mut n: i32) -> i32 {
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();

        let mut s = 0;
        for i in 1..digits.len() {
            match digits[i - 1].cmp(&digits[i]) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Less => {
                    s = i;
                }
                std::cmp::Ordering::Greater => {
                    digits[s] -= 1;
                    for n in digits[s + 1..].iter_mut() {
                        *n = 9;
                    }
                    break;
                }
            }
        }

        digits.into_iter().fold(0, |acc, x| acc * 10 + x)
        // 0ms/2.02MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
}
