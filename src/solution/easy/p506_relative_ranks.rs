struct Solution;

impl Solution {
    /// [506. 相对名次](https://leetcode.cn/problems/relative-ranks/)
    pub fn find_relative_ranks(mut score: Vec<i32>) -> Vec<String> {
        let mut vec_2d = vec![[0; 2]; score.len()];
        for i in 0..score.len() {
            vec_2d[i] = [i as i32, score[i]];
        }

        vec_2d.sort_by(|[_, l], [_, r]| r.cmp(l));

        let mut rank = vec![String::new(); score.len()];
        for i in 0..score.len() {
            rank[vec_2d[i][0] as usize] = match i + 1 {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                o => format!("{}", o),
            };
        }

        rank
        // 4ms/2.4MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            [
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string()
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            [
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string()
            ]
        );
    }
}
