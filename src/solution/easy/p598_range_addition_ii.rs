struct Solution;

impl Solution {
    /// [598. 范围求和 II](https://leetcode.cn/problems/range-addition-ii/)
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut m, mut n) = (m, n);
        for op in ops {
            if m > op[0] {
                m = op[0];
            }
            if n > op[1] {
                n = op[1];
            }
        }

        m * n
        // 0ms/2.5MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            ),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_count(3, 3, vec![]), 9);
    }
}
