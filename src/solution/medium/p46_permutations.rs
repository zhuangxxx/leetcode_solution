struct Solution;

impl Solution {
    /// [46. 全排列](https://leetcode.cn/problems/permutations/)
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            vec![nums]
        } else {
            nums.iter()
                .enumerate()
                .map(|(i, v)| {
                    Solution::permute(
                        nums.clone()
                            .into_iter()
                            .enumerate()
                            .filter_map(|(idx, val)| if idx == i { None } else { Some(val) })
                            .collect(),
                    )
                    .into_iter()
                    .map(|vec| vec![vec![*v], vec].concat())
                    .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .concat()
        }
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
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

    #[test]
    fn test2() {
        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
