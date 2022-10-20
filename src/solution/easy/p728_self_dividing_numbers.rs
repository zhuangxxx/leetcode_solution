struct Solution;

impl Solution {
    /// [728. 自除数](https://leetcode.cn/problems/self-dividing-numbers/)
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut nums = Vec::new();
        for i in left..=right {
            let mut n = i;
            while n > 0 {
                let r = n % 10;
                if r == 0 || i % r != 0 {
                    break;
                }
                if n < 10 {
                    nums.push(i);
                }
                n /= 10;
            }
        }

        nums
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }
}
