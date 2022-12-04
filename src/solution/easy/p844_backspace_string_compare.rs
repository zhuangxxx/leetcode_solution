struct Solution;

impl Solution {
    /// [844. 比较含退格的字符串](https://leetcode.cn/problems/backspace-string-compare/)
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut si = s.chars().rev().peekable();
        let mut ti = t.chars().rev().peekable();
        loop {
            let mut sp = 0;
            while sp >= 0 {
                if let Some('#') = si.peek() {
                    sp += 1;
                    si.next();
                } else {
                    if sp > 0 {
                        si.next();
                    }
                    sp -= 1;
                }
            }
            let mut tp = 0;
            while tp >= 0 {
                if let Some('#') = ti.peek() {
                    tp += 1;
                    ti.next();
                } else {
                    if tp > 0 {
                        ti.next();
                    }
                    tp -= 1;
                }
            }
            match (si.next(), ti.next()) {
                (Some(sc), Some(tc)) if sc == tc => {
                    continue;
                }
                (None, None) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        // 0ms/2.1MB
    }

    // pub fn backspace_compare(s: String, t: String) -> bool {
    //     let (mut sv, mut tv) = (Vec::new(), Vec::new());
    //     for c in s.chars() {
    //         if c == '#' {
    //             sv.pop();
    //         } else {
    //             sv.push(c);
    //         }
    //     }
    //     for c in t.chars() {
    //         if c == '#' {
    //             tv.pop();
    //         } else {
    //             tv.push(c);
    //         }
    //     }

    //     sv == tv
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::backspace_compare(
            String::from("ab#c"),
            String::from("ad#c")
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::backspace_compare(
            String::from("ab##"),
            String::from("c#d#")
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::backspace_compare(
            String::from("a#c"),
            String::from("b")
        ));
    }
}
