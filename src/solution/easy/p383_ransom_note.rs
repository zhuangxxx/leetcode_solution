struct Solution;

impl Solution {
    /// [383. 赎金信](https://leetcode.cn/problems/ransom-note/)
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut bytes = [0; 26];

        for c in magazine.chars() {
            bytes[(c as u8 - b'a') as usize] += 1;
        }

        for c in ransom_note.chars() {
            bytes[(c as u8 - b'a') as usize] -= 1;
            if bytes[(c as u8 - b'a') as usize] < 0 {
                return false;
            }
        }

        true
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::can_construct(
            String::from("a"),
            String::from("b")
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_construct(
            String::from("aa"),
            String::from("ab")
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_construct(
            String::from("aa"),
            String::from("aab")
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::can_construct(
            String::from("az"),
            String::from("ab")
        ));
    }
}
