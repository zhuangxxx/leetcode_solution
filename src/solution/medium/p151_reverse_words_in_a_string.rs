struct Solution;

impl Solution {
    /// [151. 反转字符串中的单词](https://leetcode.cn/problems/reverse-words-in-a-string/)
    pub fn reverse_words(s: String) -> String {
        let mut r = Vec::with_capacity(s.len());

        for c in s.chars().rev() {
            if c == ' ' && (r.is_empty() || r.last().is_some_and(|e| e == &' ')) {
                continue;
            }
            r.push(c);
        }
        if r.last().is_some_and(|e| e == &' ') {
            r.pop();
        }

        let (mut f, mut s) = (0, 0);
        while s < r.len() {
            if f < r.len() && r[f] != ' ' {
                f += 1;
                continue;
            }
            for i in 0..(f - s) / 2 {
                r.swap(s + i, f - i - 1);
            }
            f += 1;
            s = f;
        }

        r.into_iter().collect()
        // 0ms/2.19MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}
