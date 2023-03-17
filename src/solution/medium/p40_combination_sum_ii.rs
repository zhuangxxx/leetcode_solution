struct Solution;

impl Solution {
    /// [40. 组合总和 II](https://leetcode.cn/problems/combination-sum-ii/)
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn combine(
            candidates: &Vec<i32>,
            target: i32,
            index: usize,
            mut stack: Vec<i32>,
        ) -> Vec<Vec<i32>> {
            let mut combinations = Vec::new();
            for i in index..candidates.len() {
                if i > index && candidates[i] == candidates[i - 1] {
                    continue;
                } else if candidates[i] > target {
                    break;
                } else if candidates[i] == target {
                    stack.push(candidates[i]);
                    combinations.push(stack);
                    break;
                } else {
                    stack.push(candidates[i]);
                    combinations.append(&mut combine(
                        candidates,
                        target - candidates[i],
                        i + 1,
                        stack.clone(),
                    ));
                    stack.pop();
                }
            }

            combinations
        }

        candidates.sort_unstable();

        combine(&candidates, target, 0, Vec::new())
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
