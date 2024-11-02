struct Solution;

impl Solution {
    /// [20. 有效的括号](https://leetcode-cn.com/problems/valid-parentheses/)
    pub fn is_valid(s: String) -> bool {
        let mut stack = std::collections::VecDeque::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push_back(c),
                ')' => {
                    if let Some(c) = stack.pop_back() {
                        if c != '(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if let Some(c) = stack.pop_back() {
                        if c != '[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some(c) = stack.pop_back() {
                        if c != '{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid(String::from("()")));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_valid(String::from("()[]{}")));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid(String::from("(]")));
    }

    #[test]
    fn test4() {
        assert!(!Solution::is_valid(String::from("([)]")));
    }

    #[test]
    fn test5() {
        assert!(Solution::is_valid(String::from("{[]}")));
    }
}
