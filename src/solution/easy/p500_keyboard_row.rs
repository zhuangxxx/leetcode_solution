struct Solution;

impl Solution {
    /// [500. 键盘行](https://leetcode.cn/problems/keyboard-row/)
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        const KEYBOARD: [i8; 26] = [
            1, 2, 2, 1, 0, 1, 1, 1, 0, 1, 1, 1, 2, 2, 0, 0, 0, 0, 1, 0, 0, 2, 0, 2, 0, 2,
        ];

        let mut result = Vec::new();
        for word in words {
            let mut line = -1;
            for &u in word.as_bytes() {
                let i = (if u < b'a' { u + b'a' - b'A' } else { u } - b'a') as usize;
                if line == -1 {
                    line = KEYBOARD[i];
                } else if line != KEYBOARD[i] {
                    line = -1;
                    break;
                } else {
                    continue;
                }
            }

            if line != -1 {
                result.push(word);
            }
        }

        result
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ]),
            vec!["Alaska".to_string(), "Dad".to_string()]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_words(vec!["omk".to_string()]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_words(vec!["adsdf".to_string(), "sfd".to_string()]),
            vec!["adsdf".to_string(), "sfd".to_string()]
        );
    }
}
