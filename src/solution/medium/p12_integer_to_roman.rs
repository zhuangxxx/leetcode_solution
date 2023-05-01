struct Solution;

impl Solution {
    /// [12. 整数转罗马数字](https://leetcode.cn/problems/integer-to-roman/)
    pub fn int_to_roman(mut num: i32) -> String {
        const MAP: [(char, char); 4] = [('I', 'V'), ('X', 'L'), ('C', 'D'), ('M', '\0')];
        let mut roman = vec![String::new(); 4];
        for i in 0..roman.len() {
            let mut n = num % 10;
            if n == 4 {
                roman[i].push(MAP[i].0);
                roman[i].push(MAP[i].1);
            } else if n == 9 {
                roman[i].push(MAP[i].0);
                roman[i].push(MAP[i + 1].0);
            } else {
                if n > 4 {
                    roman[i].push(MAP[i].1);
                    n -= 5
                }
                for _ in 0..n {
                    roman[i].push(MAP[i].0);
                }
            }
            num /= 10;
        }
        roman.reverse();

        roman.concat()
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
