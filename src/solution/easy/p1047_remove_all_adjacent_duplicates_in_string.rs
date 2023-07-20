struct Solution;

impl Solution {
    /// [1047. 删除字符串中的所有相邻重复项](https://leetcode.cn/problems/remove-all-adjacent-duplicates-in-string/)
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for u in s.bytes() {
            if !stack.is_empty() && u.eq(stack.last().unwrap()) {
                stack.pop();
            } else {
                stack.push(u);
            }
        }

        String::from_utf8_lossy(&stack).into_owned()
        // 0ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::remove_duplicates("abbaca".to_string()),
            "ca".to_string()
        );
    }
}
