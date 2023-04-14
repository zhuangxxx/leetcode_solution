struct Solution;

impl Solution {
    /// [77. 组合](https://leetcode.cn/problems/combinations/)
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combine = Vec::new();
        let mut record = (1..=k).collect::<Vec<_>>();
        record.push(n + 1);

        let mut i = 0;
        while i < k as usize {
            combine.push(record[..k as usize].to_vec());

            i = 0;
            while i < k as usize && record[i] + 1 == record[i + 1] {
                record[i] = i as i32 + 1;
                i += 1;
            }

            record[i] += 1;
        }

        combine
        // 4ms/2.9MB
    }

    // pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    //     if k == 1 {
    //         (1..=n).map(|i| vec![i]).collect()
    //     } else {
    //         Self::combine(n, k - 1)
    //             .into_iter()
    //             .filter_map(|c| {
    //                 if let Some(l) = c.last() {
    //                     Some(
    //                         (l + 1..=n)
    //                             .map(|i| c.clone().into_iter().chain(std::iter::once(i)).collect())
    //                             .collect::<Vec<_>>(),
    //                     )
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .flatten()
    //             .collect()
    //     }
    //     // 104ms/27.7MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::combine(6, 3),
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 2, 6],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 3, 6],
                vec![1, 4, 5],
                vec![1, 4, 6],
                vec![1, 5, 6],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 3, 6],
                vec![2, 4, 5],
                vec![2, 4, 6],
                vec![2, 5, 6],
                vec![3, 4, 5],
                vec![3, 4, 6],
                vec![3, 5, 6],
                vec![4, 5, 6]
            ]
        );
    }
}
