struct Solution;

impl Solution {
    /// [884. 两句话中的不常见单词](https://leetcode.cn/problems/uncommon-words-from-two-sentences/)
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        format!("{} {}", s1, s2)
            .split(' ')
            .fold(std::collections::HashMap::new(), |mut map, word| {
                map.entry(word).and_modify(|count| *count += 1).or_insert(1);
                map
            })
            .iter()
            .filter_map(|(word, count)| {
                if count == &1 {
                    Some(word.to_string())
                } else {
                    None
                }
            })
            .collect()
        // 0ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        result.sort_unstable();
        assert_eq!(result, vec![String::from("sour"), String::from("sweet")]);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
            vec![String::from("banana")]
        );
    }

    #[test]
    fn fail1() {
        let mut result =
            Solution::uncommon_from_sentences("d b zu d t".to_string(), "udb zu ap".to_string());
        result.sort_unstable();
        assert_eq!(
            result,
            vec![
                String::from("ap"),
                String::from("b"),
                String::from("t"),
                String::from("udb")
            ]
        );
    }
}
