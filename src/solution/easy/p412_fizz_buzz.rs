struct Solution;

impl Solution {
    /// [412. Fizz Buzz](https://leetcode.cn/problems/fizz-buzz/)
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut vec = Vec::new();

        for i in 1..=n {
            vec.push(if i % 15 == 0 {
                String::from("FizzBuzz")
            } else if i % 5 == 0 {
                String::from("Buzz")
            } else if i % 3 == 0 {
                String::from("Fizz")
            } else {
                format!("{i}")
            })
        }

        vec
        // 0ms/2.6MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::fizz_buzz(3),
            vec![String::from("1"), String::from("2"), String::from("Fizz")]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::fizz_buzz(5),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz")
            ]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                String::from("1"),
                String::from("2"),
                String::from("Fizz"),
                String::from("4"),
                String::from("Buzz"),
                String::from("Fizz"),
                String::from("7"),
                String::from("8"),
                String::from("Fizz"),
                String::from("Buzz"),
                String::from("11"),
                String::from("Fizz"),
                String::from("13"),
                String::from("14"),
                String::from("FizzBuzz")
            ]
        );
    }
}
