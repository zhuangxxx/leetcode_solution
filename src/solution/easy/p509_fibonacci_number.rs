struct Solution;

struct Fibonacci {
    prev: i32,
    curr: i32,
}

impl std::default::Default for Fibonacci {
    fn default() -> Self {
        Self { prev: 0, curr: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev + self.curr;

        self.prev = self.curr;
        self.curr = next;

        Some(self.curr)
    }
}

impl Solution {
    /// [509. 斐波那契数](https://leetcode.cn/problems/fibonacci-number/)
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        let mut fib = Fibonacci::default();
        for _ in 1..n - 1 {
            fib.next();
        }

        fib.next().unwrap()
        // 0ms/2.2MB
    }

    // pub fn fib(n: i32) -> i32 {
    //     let sqrt5 = 5f64.sqrt();

    //     ((((1. + sqrt5) / 2.).powi(n) - ((1. - sqrt5) / 2.).powi(n)) / sqrt5).round() as i32
    //     // 0ms/2.4MB
    // }

    // pub fn fib(n: i32) -> i32 {
    //     fn tail(n: i32, prev: i32, curr: i32) -> i32 {
    //         match n {
    //             0 | 1 => n,
    //             _ => prev + tail(n - 1, curr, prev + curr),
    //         }
    //     }

    //     tail(n, 0, 1)
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::fib(30), 832040)
    }
}
