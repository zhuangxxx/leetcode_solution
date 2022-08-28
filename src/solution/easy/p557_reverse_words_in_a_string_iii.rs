struct Solution;

impl Solution {
    /// [557. 反转字符串中的单词 III](https://leetcode.cn/problems/reverse-words-in-a-string-iii/)
    pub fn reverse_words(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {
            for j in i..=chars.len() {
                if j == chars.len() || chars[j] == ' ' {
                    chars[i..j].reverse();
                    i = j + 1;
                    break;
                }
            }
        }

        chars.iter().collect()
        // 4ms/2.4MB
    }

    // pub fn reverse_words(s: String) -> String {
    //     let mut chars = s.chars().collect::<Vec<_>>();

    //     let (mut l, mut r) = (0, 0);
    //     while l < chars.len() {
    //         if r == chars.len() || chars[r] == ' ' {
    //             for i in 0..(r - l) / 2 {
    //                 chars.swap(l + i, r - i - 1);
    //             }
    //             l = r + 1;
    //         } else if l == r {
    //             l = r;
    //         }
    //         r += 1;
    //     }

    //     chars.iter().collect()
    //     // 4ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_words("God Ding".to_string()),
            "doG gniD".to_string()
        );
    }
}
