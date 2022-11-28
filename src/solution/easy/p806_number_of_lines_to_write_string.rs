struct Solution;

impl Solution {
    /// [806. 写字符串需要的行数](https://leetcode.cn/problems/number-of-lines-to-write-string/)
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let (mut line, mut curr) = (1, 0);
        for u in s.as_bytes() {
            let width = widths[(u - b'a') as usize];
            if curr + width > 100 {
                line += 1;
                curr = width;
            } else {
                curr += width;
            }
        }

        vec![line, curr]
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            vec![3, 60]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10, 10, 10, 10, 10, 10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}
