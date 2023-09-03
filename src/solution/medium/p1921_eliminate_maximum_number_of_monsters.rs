struct Solution;

impl Solution {
    /// [1921. 消灭怪物的最大数量](https://leetcode.cn/problems/eliminate-maximum-number-of-monsters/)
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let len = dist.len();

        let mut time = vec![0; len + 1];
        for t in dist
            .into_iter()
            .zip(speed)
            .map(|(d, s)| d / s + if d % s == 0 { 0 } else { 1 })
        {
            time[std::cmp::min(len, t as usize)] += 1;
        }

        let mut over = 0;
        for (i, &cost) in time[..len].iter().enumerate().skip(1) {
            if cost > 1 {
                over -= cost - 1;
            }

            if over < 0 {
                return i as i32;
            }

            if cost == 0 {
                over += 1;
            }
        }

        len as i32
        // 12ms/3.18MB
    }

    // pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    //     let mut step = dist
    //         .into_iter()
    //         .zip(speed)
    //         .map(|(d, s)| d / s + if d % s == 0 { 0 } else { 1 })
    //         .collect::<Vec<_>>();
    //     step.sort_unstable();

    //     let mut n = 0;
    //     for s in step {
    //         if s <= n {
    //             break;
    //         }

    //         n += 1;
    //     }

    //     n
    //     // 16ms/2.98MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::eliminate_maximum(vec![3, 5, 7, 4, 5], vec![2, 3, 6, 3, 2]),
            2
        );
    }
}
