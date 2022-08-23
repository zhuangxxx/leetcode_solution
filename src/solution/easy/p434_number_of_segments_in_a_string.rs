struct Solution;

impl Solution {
    /// [434. 字符串中的单词数](https://leetcode.cn/problems/number-of-segments-in-a-string/)
    pub fn count_segments(s: String) -> i32 {
        let (mut count, mut space) = (0, true);

        for c in s.chars() {
            if space && c != ' ' {
                count += 1;
                space = false;
            } else if c == ' ' {
                space = true;
            }
        }

        count
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_segments("Hello, my name is John".to_string()),
            5
        );
    }
}
