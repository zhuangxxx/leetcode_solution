struct Solution;

impl Solution {
    /// [944. 删列造序](https://leetcode.cn/problems/delete-columns-to-make-sorted/)
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut del = 0;
        for j in 0..strs[0].len() {
            let mut prev = 0;
            for i in 0..strs.len() {
                if strs[i].as_bytes()[j] < prev {
                    del += 1;
                    break;
                }

                prev = strs[i].as_bytes()[j];
            }
        }

        del
        // 0ms/2.4MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_deletion_size(vec![
                String::from("cba"),
                String::from("daf"),
                String::from("ghi")
            ]),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_deletion_size(vec![String::from("a"), String::from("b")]),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_deletion_size(vec![
                String::from("zyx"),
                String::from("wvu"),
                String::from("tsr")
            ]),
            3
        );
    }
}
