struct Solution;

impl Solution {
    /// [435. 无重叠区间](https://leetcode.cn/problems/non-overlapping-intervals/)
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let len = intervals.len() as i32;
        let mut n = 1;
        let mut r = intervals[0][1];
        for l in intervals {
            if l[0] >= r {
                n += 1;
                r = l[1];
            }
        }

        len - n
        // 38ms/10.22MB
    }

    // pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    //     intervals.sort_unstable();

    //     let mut n = 0;
    //     let (mut i, mut j) = (0, 1);
    //     while j < intervals.len() {
    //         if intervals[i][0] == intervals[j][0] || intervals[i][1] > intervals[j][0] {
    //             n += 1;
    //         }
    //         if intervals[i][1] <= intervals[j][0] || intervals[i][1] > intervals[j][1] {
    //             i = j;
    //         }
    //         j += 1;
    //     }

    //     n
    //     // 47ms/8.98MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![1, 7],
                vec![2, 3],
                vec![3, 4],
                vec![4, 8],
                vec![5, 6],
                vec![6, 7],
                vec![7, 8]
            ]),
            2
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-52, 31],
                vec![-73, -26],
                vec![82, 97],
                vec![-65, -11],
                vec![-62, -49],
                vec![95, 99],
                vec![58, 95],
                vec![-31, 49],
                vec![66, 98],
                vec![-63, 2],
                vec![30, 47],
                vec![-40, -26]
            ]),
            7
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![
                vec![-3035, 30075],
                vec![1937, 6906],
                vec![11834, 20971],
                vec![44578, 45600],
                vec![28565, 37578],
                vec![19621, 34415],
                vec![32985, 36313],
                vec![-8144, 1080],
                vec![-15279, 21851],
                vec![-27140, -14703],
                vec![-12098, 16264],
                vec![-36057, -16287],
                vec![47939, 48626],
                vec![-15129, -5773],
                vec![10508, 46685],
                vec![-35323, -26257]
            ]),
            9
        );
    }
}
