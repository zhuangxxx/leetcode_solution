struct Solution;

impl Solution {
    /// [43. 字符串相乘](https://leetcode.cn/problems/multiply-strings/)
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == *"0" || num2 == *"0" {
            return String::from("0");
        }

        let mut product = vec![0; num1.len() + num2.len()];
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let s =
                    product[i + j + 1] + (num1.as_bytes()[i] - b'0') * (num2.as_bytes()[j] - b'0');
                product[i + j + 1] = s % 10;
                product[i + j] += s / 10;
            }
        }

        product
            .drain(product.iter().position(|&u| u != 0).unwrap_or(0)..)
            .map(|u| (u + b'0') as char)
            .collect()
        // 0ms/2MB
    }

    // pub fn multiply(mut num1: String, mut num2: String) -> String {
    //     if num1 == "0".to_string() || num2 == "0".to_string() {
    //         return String::from("0");
    //     } else if num2 == "1".to_string() {
    //         return num1;
    //     } else if num1 == "1".to_string() {
    //         return num2;
    //     }
    //     if num1.len() < num2.len() {
    //         std::mem::swap(&mut num1, &mut num2);
    //     }

    //     let (mut product, mut digit) = (Vec::new(), 0);
    //     for u1 in num1.bytes().rev() {
    //         let (mut single, mut carry) = (Vec::new(), 0);
    //         for u2 in num2.bytes().rev() {
    //             let p = (u1 - b'0') * (u2 - b'0') + carry;
    //             single.push(p % 10 + b'0');
    //             carry = p / 10;
    //         }
    //         if carry > 0 {
    //             single.push(carry + b'0');
    //             carry = 0;
    //         }

    //         single = vec![vec![b'0'; digit], single].concat();
    //         if product.len() < single.len() {
    //             product.resize(single.len(), b'0');
    //         }

    //         for i in 0..product.len() {
    //             let s = (product[i] - b'0') + (single[i] - b'0') + carry;
    //             product[i] = s % 10 + b'0';
    //             carry = s / 10;
    //         }
    //         if carry > 0 {
    //             product.push(carry + b'0');
    //         }

    //         digit += 1;
    //     }

    //     product.into_iter().rev().map(|u| u as char).collect()
    //     // 4ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::multiply("9133".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
}
