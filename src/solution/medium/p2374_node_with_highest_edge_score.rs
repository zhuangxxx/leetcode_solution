struct Solution;

impl Solution {
    /// [2374. 边积分最高的节点](https://leetcode.cn/problems/node-with-highest-edge-score/)
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut scores = vec![0; edges.len()];
        for (i, edge) in edges.into_iter().enumerate() {
            scores[edge as usize] += i;
        }

        let (mut idx, mut max) = (0, 0);
        for (i, score) in scores.into_iter().enumerate() {
            if score > max {
                idx = i;
                max = score;
            }
        }

        idx as i32
        // 19ms/4.17MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::edge_score(vec![2, 0, 0, 2]), 0);
    }
}
