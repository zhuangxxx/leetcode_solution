struct Solution;

impl Solution {
    /// [242. 有效的字母异位词](https://leetcode.cn/problems/valid-anagram/)
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut bytes = [0; 26];

        let (s, t) = (s.as_bytes(), t.as_bytes());
        for i in 0..s.len() {
            bytes[(s[i] - b'a') as usize] += 1;
            bytes[(t[i] - b'a') as usize] -= 1;
        }

        bytes.iter().all(|&x| x == 0)
        // 0ms/2.2MB
    }

    // pub fn is_anagram(s: String, t: String) -> bool {
    //     let (mut s, mut t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
    //     s.sort();
    //     t.sort();

    //     s == t
    //     // 8ms/2.6MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_anagram("ac".to_string(), "bb".to_string()));
    }
}
