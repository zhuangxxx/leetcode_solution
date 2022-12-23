struct Solution;

impl Solution {
    /// [914. 卡牌分组](https://leetcode.cn/problems/x-of-a-kind-in-a-deck-of-cards/)
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        fn gcd(x: i32, y: i32) -> i32 {
            if x == 0 {
                y
            } else {
                gcd(y % x, x)
            }
        }

        let mut map = [0; 10_000];
        for card in deck {
            map[card as usize] += 1;
        }

        map.iter().fold(0, |acc, x| gcd(acc, *x)) >= 2
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
    }

    #[test]
    fn trap1() {
        assert!(!Solution::has_groups_size_x(vec![1, 2, 3, 4, 4]));
    }

    #[test]
    fn fail1() {
        assert!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
    }

    #[test]
    fn fail2() {
        assert!(Solution::has_groups_size_x(vec![
            1, 1, 1, 1, 2, 2, 2, 2, 2, 2
        ]));
    }
}
