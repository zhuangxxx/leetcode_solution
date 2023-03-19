struct Solution;

impl Solution {
    /// [47. 全排列 II](https://leetcode.cn/problems/permutations-ii/)
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            vec![nums]
        } else {
            let mut set = std::collections::HashSet::new();
            nums.iter()
                .enumerate()
                .filter_map(|(i, v)| {
                    if set.insert(v) {
                        Some(
                            Solution::permute_unique(
                                nums.clone()
                                    .into_iter()
                                    .enumerate()
                                    .filter_map(
                                        |(idx, val)| if idx == i { None } else { Some(val) },
                                    )
                                    .collect(),
                            )
                            .into_iter()
                            .map(|vec| vec![vec![*v], vec].concat())
                            .collect::<Vec<_>>(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .concat()
            // 4ms/2.1MB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
