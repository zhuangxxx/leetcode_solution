struct Solution;

impl Solution {
    /// [345. 反转字符串中的元音字母](https://leetcode.cn/problems/reverse-vowels-of-a-string/)
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();

        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            while l < s.len()
                && match s[l] {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => false,
                    _ => true,
                }
            {
                l += 1;
            }

            while r > 0
                && match s[r] {
                    'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => false,
                    _ => true,
                }
            {
                r -= 1;
            }

            if l < r {
                s.swap(l, r);
                l += 1;
                r -= 1;
            }
        }

        s.iter().collect()
        // 0ms/2.6MB
    }

    // pub fn reverse_vowels(s: String) -> String {
    //     let (mut s, mut pos) = (s.chars().collect::<Vec<_>>(), Vec::new());

    //     for i in 0..s.len() {
    //         match s[i] {
    //             'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => pos.push(i),
    //             _ => continue,
    //         }
    //     }

    //     for i in 0..pos.len() / 2 {
    //         s.swap(pos[i], pos[pos.len() - i - 1]);
    //     }

    //     s.iter().collect()
    //     // 4ms/2.8MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_string()),
            "holle".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa".to_string())
    }
}
