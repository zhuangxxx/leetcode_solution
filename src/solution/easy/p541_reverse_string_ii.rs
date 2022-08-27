struct Solution;

impl Solution {
    /// [541. 反转字符串 II](https://leetcode.cn/problems/reverse-string-ii/)
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {
            chars[i..std::cmp::min(i + k as usize, s.len())].reverse();
            i += 2 * k as usize;
        }

        chars.iter().collect()
        // 0ms/2.1MB
    }

    // pub fn reverse_str(s: String, k: i32) -> String {
    //     let mut chars = s.chars().collect::<Vec<_>>();

    //     let mut i = 0;
    //     while i < chars.len() {
    //         let mut j = std::cmp::min(i + k as usize, chars.len()) - 1;
    //         while i < j {
    //             if j < chars.len() {
    //                 chars.swap(i, j);
    //             }

    //             i += 1;
    //             j -= 1;
    //         }

    //         i += k as usize + (k as usize + 1) / 2;
    //     }

    //     chars.iter().collect()
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_str("abcdefg".to_string(), 2),
            "bacdfeg".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_str("abcd".to_string(), 2),
            "bacd".to_string()
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::reverse_str("abcdefghijklmn".to_string(), 5),
            "edcbafghijnmlk".to_string()
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::reverse_str("abcdefghijklmnop".to_string(), 6),
            "fedcbaghijklponm".to_string()
        );
    }
}
