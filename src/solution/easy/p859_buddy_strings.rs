struct Solution;

impl Solution {
    /// [859. 亲密字符串](https://leetcode.cn/problems/buddy-strings/)
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        if s == goal {
            let mut byte = [false; 26];
            for i in 0..s.len() {
                if byte[(s.as_bytes()[i] - b'a') as usize] {
                    return true;
                }

                byte[(s.as_bytes()[i] - b'a') as usize] = true;
            }

            false
        } else {
            let (mut p1, mut p2) = (None, None);
            for i in 0..s.len() {
                if s.as_bytes()[i] != goal.as_bytes()[i] {
                    if p1.is_none() {
                        p1 = Some(i);
                    } else if p2.is_none() {
                        p2 = Some(i);
                    } else {
                        return false;
                    }
                }
            }

            if let (Some(p1), Some(p2)) = (p1, p2) {
                s.as_bytes()[p1] == goal.as_bytes()[p2] && s.as_bytes()[p2] == goal.as_bytes()[p1]
            } else {
                false
            }
        }

        // 0ms/2.2MB
    }

    // pub fn buddy_strings(s: String, goal: String) -> bool {
    //     if s.len() != goal.len() {
    //         return false;
    //     }

    //     let (mut diff, mut byte) = (Vec::new(), [0u16; 26]);
    //     for i in 0..s.len() {
    //         if s.as_bytes()[i] != goal.as_bytes()[i] {
    //             if diff.len() == 2 {
    //                 return false;
    //             }

    //             diff.push((s.as_bytes()[i], goal.as_bytes()[i]));
    //         }

    //         byte[(s.as_bytes()[i] - b'a') as usize] += 1;
    //     }

    //     match diff.len() {
    //         2 => diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0,
    //         0 => byte.iter().any(|&x| x >= 2),
    //         _ => false,
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::buddy_strings(
            String::from("ab"),
            String::from("ba")
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::buddy_strings(
            String::from("ab"),
            String::from("ab")
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::buddy_strings(
            String::from("aa"),
            String::from("aa")
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::buddy_strings(
            String::from("abcaa"),
            String::from("abcbb")
        ));
    }
}
