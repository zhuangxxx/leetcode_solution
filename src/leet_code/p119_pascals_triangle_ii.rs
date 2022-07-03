struct Solution;

impl Solution {
    /// [119. 杨辉三角 II](https://leetcode.cn/problems/pascals-triangle-ii/)
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![0; row_index as usize + 1];
        row[0] = 1;

        for i in 1..row.len() {
            for j in (1..=i).rev() {
                row[j] += row[j - 1];
            }
        }

        row
        // 0ms/2MB
    }

    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     let mut row = vec![1; row_index as usize + 1];
    //     let len = row.len();

    //     for i in 2..len {
    //         let mut prev = row[0];
    //         for j in 1..std::cmp::min(i, (len + 1) / 2) {
    //             let current = row[j];

    //             row[j] = prev + current;
    //             row[len - 1 - j] = prev + current;

    //             prev = current;
    //         }
    //     }

    //     row
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}
