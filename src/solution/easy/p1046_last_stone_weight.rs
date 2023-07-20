struct Solution;

impl Solution {
    /// [1046. 最后一块石头的重量](https://leetcode.cn/problems/last-stone-weight/)
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::from(stones);

        let mut x = 0;
        while let Some(stone) = heap.pop() {
            if x != 0 {
                heap.push((stone - x).abs());
                x = 0;
            } else {
                x = stone;
            }
        }

        x
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }
}
