struct Solution;

impl Solution {
    /// [917. 仅仅反转字母](https://leetcode.cn/problems/reverse-only-letters/)
    pub fn reverse_only_letters(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            while l < r && !s[l].is_ascii_alphabetic() {
                l += 1;
            }
            while r > l && !s[r].is_ascii_alphabetic() {
                r -= 1;
            }
            if l < r && s[l].is_ascii_alphabetic() && s[r].is_ascii_alphabetic() {
                s.swap(l, r);
                l += 1;
                r -= 1;
            }
        }

        s.into_iter().collect()
        // 0ms/2MB
    }

    // pub fn reverse_only_letters(s: String) -> String {
    //     let (mut letters, mut symbols) = (Vec::new(), Vec::new());
    //     for (i, c) in s.char_indices() {
    //         if c.is_ascii_alphabetic() {
    //             letters.push(c);
    //         } else {
    //             symbols.push((i, c));
    //         }
    //     }

    //     symbols.reverse();
    //     let mut b = 0;
    //     let mut reverse = Vec::new();
    //     while let Some((i, c)) = symbols.pop() {
    //         for i in b..i {
    //             reverse.push(letters.pop().unwrap());
    //         }
    //         reverse.push(c);
    //         b = i + 1;
    //     }
    //     for i in b..s.len() {
    //         reverse.push(letters.pop().unwrap());
    //     }

    //     reverse.into_iter().collect()
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
