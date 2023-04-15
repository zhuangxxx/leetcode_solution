struct Solution;

impl Solution {
    /// [78. 子集](https://leetcode.cn/problems/subsets/)
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = Vec::new();
        for mask in 0..1 << nums.len() {
            let mut subset = Vec::new();
            for i in 0..nums.len() {
                if mask & (1 << i) != 0 {
                    subset.push(nums[i]);
                }
            }
            subsets.push(subset);
        }

        subsets
        // 0ms/2.2MB
    }

    // pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     let mut subsets = vec![Vec::new()];

    //     let mut prev = (0..nums.len()).map(|i| vec![i]).collect::<Vec<_>>();
    //     subsets.append(&mut prev.clone());
    //     for _ in 1..nums.len() {
    //         let mut subset = Vec::new();
    //         for s in prev.clone() {
    //             for i in s[s.len() - 1] + 1..nums.len() {
    //                 let mut s = s.clone();
    //                 s.push(i);
    //                 subset.push(s);
    //             }
    //         }
    //         prev = subset.clone();
    //         subsets.append(&mut subset);
    //     }

    //     subsets
    //         .into_iter()
    //         .map(|subset| subset.into_iter().map(|i| nums[i]).collect())
    //         .collect()
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
    }
}
