struct Solution;

impl Solution {
    /// [14. 最长公共前缀](https://leetcode-cn.com/problems/longest-common-prefix/)
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut min_len = strs[0].len();
        for s in &strs {
            min_len = min_len.min(s.len());
        }

        let mut i = 0;
        while i < min_len {
            let c = &strs[0].as_bytes()[i];
            for s in &strs {
                if &s.as_bytes()[i] != c {
                    return strs[0][..i].to_string();
                }
            }
            i += 1;
        }

        strs[0][..min_len].to_string()
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            String::from("fl")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            String::from("")
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flower".to_string(),
                "flower".to_string(),
                "flower".to_string()
            ]),
            String::from("flower")
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![String::from("a")]),
            String::from("a")
        );
    }
}
