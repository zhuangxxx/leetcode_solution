struct Solution;

impl Solution {
    /// [748. 最短补全词](https://leetcode.cn/problems/shortest-completing-word/)
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let plate = {
            let mut plate = [0u8; 26];
            for u in license_plate.bytes() {
                if (u >= b'a' && u <= b'z') || (u >= b'A' && u <= b'Z') {
                    plate[((u | 0x20) - b'a') as usize] += 1;
                }
            }
            plate
        };

        let mut short = String::new();
        for word in words {
            let mut temp = plate;
            for u in word.bytes() {
                if (u >= b'a' && u <= b'z') || (u >= b'A' && u <= b'Z') {
                    if temp[((u | 0x20) - b'a') as usize] == 0 {
                        continue;
                    }
                    temp[((u | 0x20) - b'a') as usize] -= 1;
                }
            }

            if temp.iter().sum::<u8>() == 0 {
                if short.is_empty() || word.len() < short.len() {
                    short = word;
                }
            }
        }

        short
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 PSt".to_string(),
                vec![
                    "step".to_string(),
                    "steps".to_string(),
                    "stripe".to_string(),
                    "stepple".to_string()
                ]
            ),
            String::from("steps")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_completing_word(
                "1s3 456".to_string(),
                vec![
                    "looks".to_string(),
                    "pest".to_string(),
                    "stew".to_string(),
                    "show".to_string()
                ]
            ),
            String::from("pest")
        );
    }
}
