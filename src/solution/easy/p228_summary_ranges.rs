struct Solution;

impl Solution {
    /// [228. 汇总区间](https://leetcode.cn/problems/summary-ranges/)
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut range = Vec::new();

        let mut i = 0;
        while i < nums.len() {
            let mut j = i;
            while j < nums.len() {
                if j == nums.len() - 1 || nums[j] + 1 != nums[j + 1] {
                    if j == i {
                        range.push(format!("{}", nums[i]));
                    } else {
                        range.push(format!("{}->{}", nums[i], nums[j]));
                    }

                    i = j + 1;
                    break;
                }

                j += 1;
            }
        }

        range
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec![
                String::from("0->2"),
                String::from("4->5"),
                String::from("7")
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                String::from("0"),
                String::from("2->4"),
                String::from("6"),
                String::from("8->9")
            ]
        );
    }
}
