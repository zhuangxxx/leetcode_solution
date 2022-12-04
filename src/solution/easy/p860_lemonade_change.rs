struct Solution;

impl Solution {
    /// [860. 柠檬水找零](https://leetcode.cn/problems/lemonade-change/)
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut purse = [0u16; 2];
        for bill in bills {
            match bill {
                10 => {
                    if purse[0] > 0 {
                        purse[0] -= 1;
                        purse[1] += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if purse[1] > 0 && purse[0] > 0 {
                        purse[1] -= 1;
                        purse[0] -= 1;
                    } else if purse[0] >= 3 {
                        purse[0] -= 3;
                    } else {
                        return false;
                    }
                }
                _ => purse[0] += 1,
            }
        }

        true
        // 12ms/2.7MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }
}
