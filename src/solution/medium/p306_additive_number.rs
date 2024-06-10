struct Solution;

impl Solution {
    /// [306. 累加数](https://leetcode.cn/problems/additive-number/)
    pub fn is_additive_number(num: String) -> bool {
        fn bt(num: &[u64], pos: usize, count: usize, prev: [u64; 2]) -> bool {
            if pos == num.len() {
                return count > 2;
            }

            let mut curr = 0;
            for i in pos..num.len() {
                if num[pos] == 0 && i > pos {
                    break;
                }

                curr = curr * 10 + num[i];
                if count >= 2 {
                    let sum = prev[0] + prev[1];
                    if curr > sum {
                        break;
                    }
                    if curr < sum {
                        continue;
                    }
                }

                if bt(num, i + 1, count + 1, [prev[1], curr]) {
                    return true;
                }
            }

            false
        }

        bt(
            &num.bytes().map(|b| (b - b'0') as u64).collect::<Vec<_>>(),
            0,
            0,
            [0; 2],
        )
        // 0ms/2.04MB
    }

    // pub fn is_additive_number(num: String) -> bool {
    //     fn bt(num: &[i32], pos: usize, prev: &mut [i32; 2]) -> bool {
    //         if pos == num.len() {
    //             return true;
    //         }

    //         let sum = prev[0] + prev[1];
    //         let mut n = 0;
    //         for i in pos..num.len() {
    //             if num[pos] == 0 && i > pos {
    //                 break;
    //             }
    //             n = n * 10 + num[i];
    //             if n == sum {
    //                 let tmp = prev[0];
    //                 prev[0] = prev[1];
    //                 prev[1] = n;
    //                 if bt(num, i + 1, prev) {
    //                     return true;
    //                 }
    //                 prev[1] = prev[0];
    //                 prev[0] = tmp;
    //                 break;
    //             }
    //         }

    //         false
    //     }

    //     if num.len() < 3 {
    //         return false;
    //     }

    //     let num = num.bytes().map(|b| (b - b'0') as i32).collect::<Vec<_>>();
    //     let mut prev = [0; 2];
    //     for i in 0..num.len() - 2 {
    //         if num[0] == 0 && i > 0 {
    //             break;
    //         }
    //         prev[0] = prev[0] * 10 + num[i];
    //         for j in i + 1..num.len() - 1 {
    //             if num[i + 1] == 0 && j > i + 1 {
    //                 break;
    //             }
    //             prev[1] = prev[1] * 10 + num[j];
    //             if bt(&num, j + 1, &mut prev.clone()) {
    //                 return true;
    //             }
    //         }
    //         prev[1] = 0;
    //     }
    //     prev[0] = 0;

    //     false
    //     // 0ms/2.12MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_additive_number(String::from("112358")));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_additive_number(String::from("199100199")));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_additive_number(String::from("111")));
    }

    #[test]
    fn fail2() {
        assert!(Solution::is_additive_number(String::from("000")));
    }

    #[test]
    fn fail3() {
        assert!(!Solution::is_additive_number(String::from("0235813")));
    }

    #[test]
    fn fail4() {
        assert!(!Solution::is_additive_number(String::from("1024")));
    }

    #[test]
    fn fail5() {
        assert!(Solution::is_additive_number(String::from(
            "121474836472147483648"
        )));
    }

    #[test]
    fn fail6() {
        assert!(Solution::is_additive_number(String::from(
            "1999999999999999910000000000000000"
        )));
    }
}
