struct Solution;

impl Solution {
    /// [796. 旋转字符串](https://leetcode.cn/problems/rotate-string/)
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && format!("{}{}", s, s).contains(&goal)
        // 0ms/2MB
    }

    // pub fn rotate_string(s: String, goal: String) -> bool {
    //     if s.len() != goal.len() {
    //         return false;
    //     }

    //     for i in 0..s.as_bytes().len() {
    //         if s.as_bytes()[i] == goal.as_bytes()[0]
    //             && s.as_bytes()[i..] == goal.as_bytes()[0..s.len() - i]
    //             && s.as_bytes()[..i] == goal.as_bytes()[s.len() - i..]
    //         {
    //             return true;
    //         }
    //     }

    //     false
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::rotate_string("aa".to_string(), "a".to_string()));
    }
}
