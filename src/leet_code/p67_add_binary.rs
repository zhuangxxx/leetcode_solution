struct Solution;

impl Solution {
    /// [67. 二进制求和](https://leetcode.cn/problems/add-binary/)
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (a.as_bytes(), b.as_bytes());

        let mut result = String::new();

        let (mut i, mut j) = ((a.len() - 1) as i32, (b.len() - 1) as i32);
        let mut carry = 0;
        while i > -1 || j > -1 {
            let va = match a.get(i as usize) {
                Some(&val) => val,
                None => 48,
            } - 48;
            let vb = match b.get(j as usize) {
                Some(&val) => val,
                None => 48,
            } - 48;

            let mut sum = va + vb + carry;
            carry = sum / 2;
            sum %= 2;
            result.insert(0, char::from(sum + 48));

            i -= 1;
            j -= 1;
        }

        if carry > 0 {
            result.insert(0, char::from(49));
        }

        result
    }

    // pub fn add_binary(a: String, b: String) -> String {
    //     let (mut a, mut b) = (
    //         a.chars().rev().collect::<Vec<_>>(),
    //         b.chars().rev().collect::<Vec<_>>(),
    //     );

    //     let mut pad = vec!['0'; (a.len() as i32 - b.len() as i32).abs() as usize];
    //     if a.len() > b.len() {
    //         b.append(&mut pad);
    //     } else {
    //         a.append(&mut pad);
    //     }

    //     let (mut result, mut carry) = (Vec::new(), 0);

    //     for i in 0..a.len() {
    //         if a[i] == '1' && b[i] == '1' {
    //             if carry > 0 {
    //                 result.push('1');
    //                 carry -= 1;
    //             } else {
    //                 result.push('0');
    //             }
    //             carry += 1;
    //         } else if a[i] == '1' || b[i] == '1' {
    //             if carry > 0 {
    //                 result.push('0');
    //             } else {
    //                 result.push('1');
    //             }
    //         } else {
    //             if carry > 0 {
    //                 result.push('1');
    //                 carry -= 1;
    //             } else {
    //                 result.push('0');
    //             }
    //         }
    //     }

    //     if carry > 0 {
    //         result.push('1');
    //     }

    //     result.reverse();
    //     result.iter().collect::<String>()
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            String::from("100")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            String::from("10101")
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::add_binary("1111".to_string(), "1111".to_string()),
            String::from("11110")
        );
    }
}
