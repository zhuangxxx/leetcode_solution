struct Solution;

impl Solution {
    /// [942. 增减字符串匹配](https://leetcode.cn/problems/di-string-match/)
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut v = Vec::with_capacity(s.len() + 1);

        let (mut l, mut r) = (0, s.len() as i32);
        for c in s.chars() {
            if c == 'I' {
                v.push(l);
                l += 1;
            } else {
                v.push(r);
                r -= 1;
            }
        }
        v.push(l);

        v
        // 4ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::di_string_match("IDID".to_string()),
            vec![0, 4, 1, 3, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::di_string_match("III".to_string()),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::di_string_match("DDI".to_string()),
            vec![3, 2, 0, 1]
        );
    }
}
