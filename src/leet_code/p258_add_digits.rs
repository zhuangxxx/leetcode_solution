struct Solution;

impl Solution {
    /// [258. 各位相加](https://leetcode.cn/problems/add-digits/)
    pub fn add_digits(num: i32) -> i32 {
        let (mut sum, mut num) = (0, num);

        loop {
            sum += num % 10;
            num /= 10;
            if num == 0 {
                std::mem::swap(&mut num, &mut sum);
                if num < 10 {
                    break;
                }
            }
        }

        num
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::add_digits(0), 0);
    }
}
