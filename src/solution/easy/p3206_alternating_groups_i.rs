struct Solution;

impl Solution {
    /// [3206. 交替组 I](https://leetcode.cn/problems/alternating-groups-i/)
    pub fn number_of_alternating_groups(colors: Vec<i32>) -> i32 {
        colors.windows(3).fold(0, |acc, w| {
            if w[0] == w[2] && w[0] != w[1] {
                acc + 1
            } else {
                acc
            }
        }) + if colors[colors.len() - 1] == colors[1] && colors[0] != colors[1] {
            1
        } else {
            0
        } + if colors[colors.len() - 2] == colors[0] && colors[colors.len() - 1] != colors[0] {
            1
        } else {
            0
        }
        // 0ms/2.36MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::number_of_alternating_groups(vec![1, 1, 1]), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1]),
            3
        );
    }
}
