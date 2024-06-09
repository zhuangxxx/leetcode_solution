struct Solution;

impl Solution {
    /// [452. 用最少数量的箭引爆气球](https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/)
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if (points.is_empty()) {
            return 0;
        }

        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut shots = 1;
        let mut e = points[0][1];
        for balloon in points {
            if balloon[0] > e {
                e = balloon[1];
                shots += 1;
            }
        }

        shots
        // 37ms/9.69MB
    }

    // pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    //     points.sort_unstable();

    //     let mut shots = 0;
    //     let mut i = 0;
    //     while i < points.len() {
    //         let mut e = points[i][1];
    //         let mut j = i + 1;
    //         while j < points.len() && e >= points[j][0] {
    //             e = std::cmp::min(e, points[j][1]);
    //             j += 1;
    //         }
    //         i = j;
    //         shots += 1;
    //     }

    //     shots
    //     // 40ms/9.00MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![
                vec![9, 12],
                vec![1, 10],
                vec![4, 11],
                vec![8, 12],
                vec![3, 9],
                vec![6, 9],
                vec![6, 7]
            ]),
            2
        );
    }
}
