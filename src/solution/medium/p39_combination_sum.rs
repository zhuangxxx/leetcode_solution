struct Solution;

impl Solution {
    /// [39. 组合总和](https://leetcode.cn/problems/combination-sum/)
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            candidates: &Vec<i32>,
            target: i32,
            index: usize,
            mut combine: Vec<i32>,
        ) -> Vec<Vec<i32>> {
            if index == candidates.len() {
                return Vec::new();
            }

            if target == 0 {
                return vec![combine];
            }

            let mut combinations = dfs(candidates, target, index + 1, combine.clone());
            if target - candidates[index] >= 0 {
                combine.push(candidates[index]);
                combinations.append(&mut dfs(
                    candidates,
                    target - candidates[index],
                    index,
                    combine.clone(),
                ));
                combine.pop();
            }

            combinations
        }

        dfs(&candidates, target, 0, Vec::new())
        // 4ms/2.3MB
    }

    // pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    //     fn combine(
    //         candidates: &Vec<i32>,
    //         target: i32,
    //         index: usize,
    //         mut stack: Vec<i32>,
    //     ) -> Vec<Vec<i32>> {
    //         let mut combinations = Vec::new();
    //         for i in index..candidates.len() {
    //             if candidates[i] > target {
    //                 break;
    //             } else if candidates[i] == target {
    //                 stack.push(candidates[i]);
    //                 combinations.push(stack);
    //                 break;
    //             } else {
    //                 stack.push(candidates[i]);
    //                 combinations.append(&mut combine(
    //                     candidates,
    //                     target - candidates[i],
    //                     i,
    //                     stack.clone(),
    //                 ));
    //                 stack.pop();
    //             }
    //         }

    //         combinations
    //     }

    //     candidates.sort_unstable();

    //     combine(&candidates, target, 0, Vec::new())
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        result.sort_unstable();
        assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test2() {
        let mut result = Solution::combination_sum(vec![2, 3, 5], 8);
        result.sort_unstable();
        assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::combination_sum(vec![2], 1), Vec::<Vec<_>>::new());
    }
}
