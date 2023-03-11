struct Solution;

impl Solution {
    /// [38. 外观数列](https://leetcode.cn/problems/count-and-say/)
    pub fn count_and_say(n: i32) -> String {
        let mut s = String::from("1");
        for _ in 1..n {
            let mut sv = Vec::new();
            let (mut count, mut curr) = (0, s.as_bytes()[0] as char);
            for c in s.chars() {
                if curr == c {
                    count += 1;
                } else {
                    sv.push(format!("{}{}", count, curr));
                    count = 1;
                    curr = c;
                }
            }
            sv.push(format!("{}{}", count, curr));
            s = sv.concat();
        }

        s
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_and_say(1), String::from("1"));
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_and_say(4), String::from("1211"));
    }
}
