struct Solution;

impl Solution {
    /// [551. 学生出勤记录 I](https://leetcode.cn/problems/student-attendance-record-i/)
    pub fn check_record(s: String) -> bool {
        let (mut na, mut ncl) = (0, 0);
        for c in s.chars() {
            match c {
                'A' => {
                    na += 1;
                    if na > 1 {
                        return false;
                    }
                    ncl = 0;
                }
                'L' => {
                    ncl += 1;
                    if ncl > 2 {
                        return false;
                    }
                }
                _ => ncl = 0,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_record(String::from("PPALLP")));
    }

    #[test]
    fn test2() {
        assert!(!Solution::check_record(String::from("PPALLL")));
    }

    #[test]
    fn trap1() {
        assert!(Solution::check_record(String::from("PLLALL")));
    }
}
