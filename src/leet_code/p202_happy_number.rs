struct Solution;

impl Solution {
    /// [202. 快乐数](https://leetcode.cn/problems/happy-number/)
    pub fn is_happy(n: i32) -> bool {
        fn next(n: i32) -> i32 {
            let (mut n, mut x) = (n, 0);
            while n > 0 {
                x += (n % 10) * (n % 10);
                n /= 10;
            }

            x
        }

        let (mut slow, mut fast) = (n, next(n));

        while n != 1 && slow != fast {
            slow = next(slow);
            fast = next(next(fast));
        }

        slow == 1
        // 0ms/2.1MB
    }

    // pub fn is_happy(n: i32) -> bool {
    //     let mut x = n;

    //     loop {
    //         let mut y = 0;
    //         while x > 0 {
    //             y += (x % 10) * (x % 10);
    //             x /= 10;
    //         }
    //         if y == 1 {
    //             return true;
    //         }
    //         if y == n || y == 4 {
    //             return false;
    //         }

    //         x = y;
    //     }

    //     false
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn trap1() {
        assert!(!Solution::is_happy(i32::MAX));
    }
}
