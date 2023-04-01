struct Solution;

impl Solution {
    /// [57. 插入区间](https://leetcode.cn/problems/insert-interval/)
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        fn binary_search(intervals: &Vec<Vec<i32>>, target: i32, i: usize) -> (usize, i32) {
            let (mut l, mut r) = (0, intervals.len() - 1);
            while l <= r {
                let m = l + (r - l) / 2;
                if intervals[m][1] < target {
                    if m == intervals.len() - 1 {
                        return (m, target);
                    } else if intervals[m + 1][0] > target {
                        return (m + if i == 0 { 1 } else { 0 }, target);
                    }
                    l = m + 1;
                } else if intervals[m][0] > target {
                    if m == 0 {
                        return (m, target);
                    } else if intervals[m - 1][1] < target {
                        return (m - if i == 1 { 1 } else { 0 }, target);
                    }
                    r = m;
                } else {
                    return (m, intervals[m][i]);
                }
            }

            (intervals.len(), target)
        }

        if intervals.is_empty() {
            return vec![new_interval];
        }
        if intervals[0][0] > new_interval[1] {
            intervals.insert(0, new_interval);
            return intervals;
        }
        if intervals[intervals.len() - 1][1] < new_interval[0] {
            intervals.push(new_interval);
            return intervals;
        }

        let (l, r) = (
            if intervals[0][0] >= new_interval[0] {
                (0, new_interval[0])
            } else {
                binary_search(&intervals, new_interval[0], 0)
            },
            if intervals[intervals.len() - 1][1] <= new_interval[1] {
                (intervals.len() - 1, new_interval[1])
            } else {
                binary_search(&intervals, new_interval[1], 1)
            },
        );

        for _ in l.0..=r.0 {
            intervals.remove(l.0);
        }
        intervals.insert(l.0, vec![l.1, r.1]);

        intervals
        // 352ms/2.7MB
    }

    // pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    //     if intervals.is_empty() {
    //         return vec![new_interval];
    //     }

    //     let (mut l, mut r) = ((false, 0), (false, 0));
    //     for i in 0..intervals.len() {
    //         if !l.0 {
    //             if new_interval[0] <= intervals[i][1] {
    //                 if new_interval[0] >= intervals[i][0] || i == 0 {
    //                     l = (true, i);
    //                 }
    //             } else if i == intervals.len() - 1 {
    //                 intervals.push(new_interval);
    //                 return intervals;
    //             } else if new_interval[0] < intervals[i + 1][0] {
    //                 l = (true, i + 1);
    //             }
    //         }
    //         if !r.0 {
    //             if new_interval[1] >= intervals[i][0] {
    //                 if new_interval[1] <= intervals[i][1] || i == intervals.len() - 1 {
    //                     r = (true, i);
    //                 }
    //             } else if i == 0 {
    //                 intervals.insert(0, new_interval);
    //                 return intervals;
    //             } else if new_interval[1] > intervals[i - 1][1] {
    //                 r = (true, i - 1);
    //             }
    //         }
    //         if l.0 && r.0 {
    //             break;
    //         }
    //     }

    //     if l.0 && r.0 {
    //         if l.1 > r.1 {
    //             intervals.insert(l.1, new_interval);
    //         } else {
    //             new_interval[0] = std::cmp::min(intervals[l.1][0], new_interval[0]);
    //             new_interval[1] = std::cmp::max(intervals[r.1][1], new_interval[1]);
    //             for _ in l.1..=r.1 {
    //                 intervals.remove(l.1);
    //             }
    //             intervals.insert(l.1, new_interval);
    //         }
    //     }

    //     intervals
    //     // 352ms/2.6MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 3]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 7]),
            vec![vec![1, 7]]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::insert(vec![vec![1, 5], vec![6, 8]], vec![0, 9]),
            vec![vec![0, 9]]
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::insert(vec![vec![3, 4], vec![5, 6]], vec![1, 2]),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::insert(vec![vec![3, 4], vec![5, 6]], vec![7, 8]),
            vec![vec![3, 4], vec![5, 6], vec![7, 8]]
        );
    }
}
