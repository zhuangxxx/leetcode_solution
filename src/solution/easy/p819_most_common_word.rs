struct Solution;

impl Solution {
    /// [819. 最常见的单词](https://leetcode.cn/problems/most-common-word/)
    pub fn most_common_word(mut paragraph: String, banned: Vec<String>) -> String {
        let (mut result, mut count) = ("", 0);
        let mut map = std::collections::HashMap::new();

        paragraph = paragraph.to_ascii_lowercase();
        for word in paragraph.split_terminator(&[' ', '!', '?', '\'', ',', ';', '.'][..]) {
            if !word.is_empty() && !banned.contains(&word.to_owned()) {
                let mut entry = map.entry(word).or_insert(1);
                *entry += 1;

                if (entry > &mut count) {
                    result = word;
                    count = *entry;
                }
            }
        }

        result.to_string()
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            String::from("ball")
        );
    }
}
