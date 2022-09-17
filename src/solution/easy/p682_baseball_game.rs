struct Solution;

impl Solution {
    /// [682. 棒球比赛](https://leetcode.cn/problems/baseball-game/)
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut points = Vec::new();

        let mut i = 0;
        for operation in operations {
            match operation.as_str() {
                "+" => points.push(points[i - 2] + points[i - 1]),
                "D" => points.push(points[i - 1] * 2),
                "C" => {
                    points.pop();
                    i -= 2;
                }
                point => points.push(point.parse::<i32>().unwrap_or_default()),
            }

            i += 1;
        }

        points.iter().sum()
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ]),
            27
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::cal_points(vec!["1".to_string()]), 1);
    }
}
