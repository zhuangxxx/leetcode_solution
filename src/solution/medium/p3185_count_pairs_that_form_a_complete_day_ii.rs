struct Solution;

impl Solution {
    /// [3185. 构成整天的下标对数目 II](https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/)
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
        let mut s = 0;

        let mut map = [0; 24];
        for hour in hours {
            let rem = hour as usize % 24;
            s += map[if rem == 0 { 0 } else { 24 - rem }];
            map[rem] += 1;
        }

        s
        // 4ms/6.16MB
    }

    // pub fn count_complete_day_pairs(hours: Vec<i32>) -> i64 {
    //     let mut map = std::collections::HashMap::new();
    //     for hour in hours {
    //         map.entry(match hour % 24 {
    //             0 => 0,
    //             rem => 24 - rem,
    //         })
    //         .and_modify(|x| *x += 1)
    //         .or_insert(1);
    //     }

    //     let mut s = 0;

    //     let mut v = std::collections::HashSet::new();
    //     for k in map.keys() {
    //         if v.contains(k) {
    //             continue;
    //         }

    //         s += match k {
    //             0 | 12 => {
    //                 if map[k] > 1 {
    //                     map[k] * (map[k] - 1) >> 1
    //                 } else {
    //                     0
    //                 }
    //             }
    //             _ => {
    //                 v.insert(24 - k);
    //                 if let Some(n) = map.get(&(24 - k)) {
    //                     n * map[k]
    //                 } else {
    //                     0
    //                 }
    //             }
    //         };
    //     }

    //     s
    //     // 15ms/6.09MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
