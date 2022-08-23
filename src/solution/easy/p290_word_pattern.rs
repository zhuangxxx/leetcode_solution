struct Solution;

impl Solution {
    /// [290. 单词规律](https://leetcode.cn/problems/word-pattern/)
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let (pattern, s) = (pattern.as_bytes(), s.split(' ').collect::<Vec<_>>());
        if pattern.len() != s.len() {
            return false;
        }

        let (mut map, mut set) = (
            std::collections::HashMap::new(),
            std::collections::HashSet::new(),
        );
        for i in 0..s.len() {
            if map.contains_key(&pattern[i]) {
                if map[&pattern[i]] != s[i] {
                    return false;
                }
            } else {
                map.insert(pattern[i], s[i]);
                if !set.insert(s[i]) {
                    return false;
                }
            }
        }

        true
        // 0ms/2MB
    }

    // pub fn word_pattern(pattern: String, s: String) -> bool {
    //     let s = s.split(' ').collect::<Vec<_>>();
    //     let p_set_len = pattern
    //         .as_bytes()
    //         .iter()
    //         .collect::<std::collections::HashSet<_>>()
    //         .len();
    //     let s_set_len = s.iter().collect::<std::collections::HashSet<_>>().len();
    //     let zip_set_len = pattern
    //         .as_bytes()
    //         .iter()
    //         .zip(s.iter())
    //         .collect::<std::collections::HashSet<_>>()
    //         .len();

    //     pattern.len() == s.len() && p_set_len == s_set_len && s_set_len == zip_set_len
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::word_pattern(
            String::from("abba"),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::word_pattern(
            String::from("abba"),
            "dog cat cat fish".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::word_pattern(
            String::from("aaaa"),
            "dog cat cat dog".to_string()
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::word_pattern(
            String::from("abba"),
            "dog dog dog dog".to_string()
        ));
    }
}
