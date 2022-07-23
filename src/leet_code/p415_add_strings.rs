struct Solution;

impl Solution {
    /// [415. 字符串相加](https://leetcode.cn/problems/add-strings/)
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, num2) = if num1.len() > num2.len() {
            let mut num2 = num2.bytes().rev().collect::<Vec<_>>();
            num2.resize(num1.len(), b'0');
            (num1.bytes().rev().collect::<Vec<_>>(), num2)
        } else {
            let mut num1 = num1.bytes().rev().collect::<Vec<_>>();
            num1.resize(num2.len(), b'0');
            (num1, num2.bytes().rev().collect::<Vec<_>>())
        };

        let (mut sum, mut one) = (vec![0; num1.len()], 0);
        for i in 0..num2.len() {
            sum[i] = num1[i] + num2[i] + one - b'0' * 2;
            if sum[i] > 9 {
                sum[i] %= 10;
                one = 1;
            } else {
                one = 0;
            }
            sum[i] += b'0';
        }

        if one == 1 {
            sum.push(b'1');
        }

        sum.reverse();

        if let Ok(sum) = String::from_utf8(sum) {
            sum
        } else {
            String::from("0")
        }
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_strings(String::from("11"), String::from("123")),
            String::from("134")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_strings(String::from("456"), String::from("77")),
            String::from("533")
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::add_strings(String::from("0"), String::from("0")),
            String::from("0")
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::add_strings(String::from("9"), String::from("1")),
            String::from("10")
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::add_strings(String::from("9999999999999999999"), String::from("1")),
            String::from("10000000000000000000")
        );
    }
}
