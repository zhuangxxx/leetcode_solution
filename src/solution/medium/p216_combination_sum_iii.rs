struct Solution;

impl Solution {
    /// [216. 组合总和 III](https://leetcode.cn/problems/combination-sum-iii/)
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn bt(k: i32, n: i32, i: i32, s: i32, combine: &mut Vec<i32>) -> Vec<Vec<i32>> {
            if k == 0 {
                return if s == n {
                    vec![combine.clone()]
                } else {
                    Vec::new()
                };
            }

            let mut result = Vec::new();
            for i in i + 1..=std::cmp::min(n - s, 9) {
                combine.push(i);
                result.append(&mut bt(k - 1, n, i, s + i, combine));
                combine.pop();
            }

            result
        }

        bt(k, n, 0, 0, &mut Vec::new())
        // 0ms/2.20MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::combination_sum3(4, 1), Vec::<Vec<_>>::new());
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::combination_sum3(2, 18), Vec::<Vec<_>>::new());
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::combination_sum3(9, 45),
            vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]
        );
    }
}
