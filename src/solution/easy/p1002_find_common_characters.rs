struct Solution;

impl Solution {
    /// [1002. 查找共用字符](https://leetcode.cn/problems/find-common-characters/)
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut map = [words[0].len(); 26];
        for w in words {
            let mut m = [0; 26];
            for u in w.bytes() {
                m[(u - b'a') as usize] += 1;
            }
            for i in 0..26 {
                if map[i] > 0 && m[i] != map[i] {
                    map[i] = std::cmp::min(m[i], map[i]);
                }
            }
        }

        let mut common = Vec::new();
        for (i, c) in map.iter().enumerate() {
            for _ in 0..*c {
                common.push(format!("{}", (b'a' + i as u8) as char));
            }
        }

        common
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::common_chars(vec![
                "bella".to_string(),
                "label".to_string(),
                "roller".to_string()
            ]),
            vec![String::from("e"), String::from("l"), String::from("l")]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::common_chars(vec![
                "cool".to_string(),
                "lock".to_string(),
                "cook".to_string()
            ]),
            vec![String::from("c"), String::from("o")]
        );
    }
}
