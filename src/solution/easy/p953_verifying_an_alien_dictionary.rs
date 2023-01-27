struct Solution;

impl Solution {
    /// [953. 验证外星语词典](https://leetcode.cn/problems/verifying-an-alien-dictionary/)
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = [0; 26];
        let mut i = 0u8;
        for b in order.bytes() {
            map[(b - b'a') as usize] = i;
            i += 1;
        }

        for i in 1..words.len() {
            for j in 0..std::cmp::max(words[i - 1].len(), words[i].len()) {
                if j == words[i].len() {
                    return false;
                } else if j == words[i - 1].len() {
                    break;
                }

                if map[(words[i - 1].as_bytes()[j] - b'a') as usize]
                    > map[(words[i].as_bytes()[j] - b'a') as usize]
                {
                    return false;
                } else if map[(words[i - 1].as_bytes()[j] - b'a') as usize]
                    < map[(words[i].as_bytes()[j] - b'a') as usize]
                {
                    break;
                } else {
                    continue;
                }
            }
        }

        true
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ));
    }
}
