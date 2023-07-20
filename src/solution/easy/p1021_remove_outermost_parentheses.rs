struct Solution;

impl Solution {
    /// [1021. 删除最外层的括号](https://leetcode.cn/problems/remove-outermost-parentheses/)
    pub fn remove_outer_parentheses(s: String) -> String {
        s.chars()
            .scan(0, |count, c| {
                *count += if c == '(' { 1 } else { -1 };
                Some(if c == '(' && *count > 1 || (c == ')' && *count > 0) {
                    c
                } else {
                    '\0'
                })
            })
            .filter(|c| '\0'.ne(c))
            .collect()
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::remove_outer_parentheses("()()".to_string()),
            String::new()
        );
    }
}
