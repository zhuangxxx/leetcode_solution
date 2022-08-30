struct Solution;

impl Solution {
    /// [575. 分糖果](https://leetcode.cn/problems/distribute-candies/)
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        std::cmp::min(
            candy_type
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            candy_type.len() / 2,
        ) as i32
        // 32ms/2.2MB
    }

    // pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    //     let mut set = std::collections::HashSet::new();

    //     let len = candy_type.len();
    //     for candy in candy_type {
    //         set.insert(candy);
    //     }

    //     std::cmp::min(set.len(), len / 2) as i32
    //     // 48ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
