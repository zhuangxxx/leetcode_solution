struct Solution;

impl Solution {
    /// [804. 唯一摩尔斯密码词](https://leetcode.cn/problems/unique-morse-code-words/)
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        const MORSE: [[u64; 2]; 26] = [
            [0b01, 2],
            [0b1000, 4],
            [0b1010, 4],
            [0b100, 3],
            [0b0, 1],
            [0b0010, 4],
            [0b110, 3],
            [0b0000, 4],
            [0b00, 2],
            [0b0111, 4],
            [0b101, 3],
            [0b0100, 4],
            [0b11, 2],
            [0b10, 2],
            [0b111, 3],
            [0b0110, 4],
            [0b1101, 4],
            [0b010, 3],
            [0b000, 3],
            [0b1, 1],
            [0b001, 3],
            [0b0001, 4],
            [0b011, 3],
            [0b1001, 4],
            [0b1011, 4],
            [0b1100, 4],
        ];

        let mut set = std::collections::HashSet::new();
        for word in words {
            let mut mask = 1;
            for u in word.as_bytes() {
                mask = mask << MORSE[(u - b'a') as usize][1] | MORSE[(u - b'a') as usize][0];
            }

            set.insert(mask);
        }

        set.len() as i32
        // 0ms/2.1MB
    }

    // pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    //     const MORSE: [&str; 26] = [
    //         ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    //         "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
    //         "-.--", "--..",
    //     ];

    //     let mut set = std::collections::HashSet::new();
    //     for word in words {
    //         let mut code = String::new();
    //         for u in word.as_bytes() {
    //             code.push_str(MORSE[(u - b'a') as usize]);
    //         }

    //         set.insert(code);
    //     }

    //     set.len() as i32
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::unique_morse_representations(vec![
                "gin".to_string(),
                "zen".to_string(),
                "gig".to_string(),
                "msg".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::unique_morse_representations(vec![String::from("a")]),
            1
        );
    }
}
