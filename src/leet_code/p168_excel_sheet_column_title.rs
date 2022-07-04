struct Solution;

impl Solution {
    /// [168. Excel表列名称](https://leetcode.cn/problems/excel-sheet-column-title/)
    pub fn convert_to_title(column_number: i32) -> String {
        let mut title_vec = Vec::new();

        let mut col_num = column_number;
        while col_num > 0 {
            col_num -= 1;
            title_vec.push((col_num % 26) as u8 + 65);
            col_num /= 26;
        }

        title_vec.reverse();
        if let Ok(title) = String::from_utf8(title_vec) {
            title
        } else {
            String::new()
        }
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::convert_to_title(1), "A");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::convert_to_title(2147483647), "FXSHRXW");
    }
}
