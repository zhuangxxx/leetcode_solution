struct Solution;

impl Solution {
    /// [28. 实现 strStr()](https://leetcode.cn/problems/implement-strstr/)
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());

        if needle.is_empty() {
            return 0;
        }

        let mut next = vec![0; needle.len()];
        let mut j = 0;

        for i in 1..needle.len() {
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }

            if needle[i] == needle[j] {
                j += 1;
            }

            next[i] = j;
        }

        let mut j = 0;

        for i in 0..haystack.len() {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }

            if haystack[i] == needle[j] {
                j += 1;
            }

            if j == needle.len() {
                return i as i32 - needle.len() as i32 + 1;
            }
        }

        -1
        // 0ms/2MB
    }

    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());

    //     if needle.is_empty() {
    //         return 0;
    //     }

    //     if haystack.len() < needle.len() {
    //         return -1;
    //     }

    //     let mut offset = std::collections::HashMap::new();

    //     for i in 0..needle.len() {
    //         offset.insert(needle[i], needle.len() - i);
    //     }

    //     let mut i = 0;

    //     while i <= haystack.len() - needle.len() {
    //         if &haystack[i..i + needle.len()] == needle {
    //             return i as i32;
    //         }

    //         if i + needle.len() > haystack.len() - 1 {
    //             return -1;
    //         }

    //         if offset.get(&haystack[i + needle.len()]).is_some() {
    //             i += offset[&haystack[i + needle.len()]];
    //         } else {
    //             i += 1;
    //         }
    //     }

    //     -1
    //     // 0ms/2MB
    // }

    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());

    //     if needle.is_empty() {
    //         return 0;
    //     }

    //     let (mut i, mut j) = (0, 0);

    //     while i < haystack.len() {
    //         if haystack[i] == needle[j] {
    //             if j == needle.len() - 1 {
    //                 return i as i32 - j as i32;
    //             }

    //             j += 1;
    //         } else {
    //             i -= j;
    //             j = 0;
    //         }

    //         i += 1;
    //     }

    //     -1
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
    }

    #[test]
    fn test3() {
        println!("{}", Solution::str_str("a".to_string(), "xxx".to_string()));
    }
}
