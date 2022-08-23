struct Solution;

impl Solution {
    /// [171. Excel 表列序号](https://leetcode.cn/problems/excel-sheet-column-number/)
    pub fn title_to_number(column_title: String) -> i32 {
        let column_title = column_title.as_bytes();
        let mut num = 0;

        for &col in column_title {
            num *= 26;
            num += col as i32 - 64;
        }

        num
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::title_to_number(String::from("A")), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::title_to_number(String::from("AB")), 28);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::title_to_number(String::from("FXSHRXW")), i32::MAX);
    }
}
