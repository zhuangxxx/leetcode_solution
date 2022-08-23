struct Solution;

impl Solution {
    /// [389. 找不同](https://leetcode.cn/problems/find-the-difference/)
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut xor = 0;

        for c in s.chars() {
            xor ^= c as u8;
        }

        for c in t.chars() {
            xor ^= c as u8;
        }

        xor as char
        // 0ms/2.3MB
    }

    // pub fn find_the_difference(s: String, t: String) -> char {
    //     let (mut ss, mut ts) = (0, 0);

    //     for c in s.chars() {
    //         ss += c as u8 as i32;
    //     }

    //     for c in t.chars() {
    //         ts += c as u8 as i32;
    //     }

    //     (ts - ss) as u8 as char
    //     // 0ms/2.3MB
    // }

    // pub fn find_the_difference(s: String, t: String) -> char {
    //     let mut bytes = [0; 26];

    //     let (s, t) = (s.as_bytes(), t.as_bytes());
    //     for i in 0..s.len() {
    //         bytes[(s[i] - b'a') as usize] += 1;
    //         bytes[(t[i] - b'a') as usize] -= 1;
    //     }

    //     bytes[(t[s.len()] - b'a') as usize] -= 1;

    //     for i in 0..26 {
    //         if bytes[i] != 0 {
    //             return (i as u8 + b'a') as char;
    //         }
    //     }

    //     t[s.len()] as char
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_the_difference(String::from("abcd"), String::from("abcde")),
            'e'
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_the_difference(String::new(), String::from("y")),
            'y'
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_the_difference(String::from("abc"), String::from("abdc")),
            'd'
        )
    }
}
