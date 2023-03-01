struct Solution;

impl Solution {
    /// [17. 电话号码的字母组合](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/)
    pub fn letter_combinations(digits: String) -> Vec<String> {
        const LETTER: [(u8, u8); 8] = [
            (b'a', 3),
            (b'd', 3),
            (b'g', 3),
            (b'j', 3),
            (b'm', 3),
            (b'p', 4),
            (b't', 3),
            (b'w', 4),
        ];

        if digits.is_empty() {
            return Vec::new();
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(String::new());
        for u in digits.bytes() {
            for _ in 0..queue.len() {
                if let Some(s) = queue.pop_front() {
                    for i in 0..LETTER[(u - b'2') as usize].1 {
                        queue.push_back(format!(
                            "{}{}",
                            s,
                            (LETTER[(u - b'2') as usize].0 + i) as char
                        ));
                    }
                }
            }
        }

        queue.into()
        // 0ms/1.9MB
    }

    // pub fn letter_combinations(digits: String) -> Vec<String> {
    //     const LETTER: [(u8, u8); 8] = [
    //         (b'a', 3),
    //         (b'd', 3),
    //         (b'g', 3),
    //         (b'j', 3),
    //         (b'm', 3),
    //         (b'p', 4),
    //         (b't', 3),
    //         (b'w', 4),
    //     ];

    //     fn letter_combination(
    //         digits: &[(u8, u8)],
    //         index: usize,
    //         combination: String,
    //     ) -> Vec<String> {
    //         if index == digits.len() {
    //             return vec![combination];
    //         }

    //         let mut combinations = Vec::new();
    //         for i in 0..digits[index].1 {
    //             combinations.append(&mut letter_combination(
    //                 digits,
    //                 index + 1,
    //                 format!("{}{}", combination, (digits[index].0 + i) as char),
    //             ));
    //         }

    //         combinations
    //     }

    //     if digits.is_empty() {
    //         return Vec::new();
    //     }

    //     let digits = digits
    //         .bytes()
    //         .map(|u| LETTER[(u - b'2') as usize])
    //         .collect::<Vec<_>>();

    //     letter_combination(&digits, 0, String::new())
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec![
                String::from("ad"),
                String::from("ae"),
                String::from("af"),
                String::from("bd"),
                String::from("be"),
                String::from("bf"),
                String::from("cd"),
                String::from("ce"),
                String::from("cf")
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::letter_combinations(String::from("")),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            vec![String::from("a"), String::from("b"), String::from("c")]
        );
    }
}
