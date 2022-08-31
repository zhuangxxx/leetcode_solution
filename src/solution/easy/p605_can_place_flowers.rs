struct Solution;

impl Solution {
    /// [605. 种花问题](https://leetcode.cn/problems/can-place-flowers/)
    pub fn can_place_flowers(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut i = 0;
        while i < flowerbed.len() {
            if flowerbed[i] == 0 {
                if i == flowerbed.len() - 1 || flowerbed[i + 1] == 0 {
                    n -= 1;
                } else {
                    i += 1;
                }
            }

            i += 2;
        }

        n <= 0
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn trap1() {
        assert!(Solution::can_place_flowers(vec![0], 1));
    }

    #[test]
    fn trap2() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 0], 2));
    }

    #[test]
    fn trap3() {
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 3));
    }

    #[test]
    fn fail1() {
        assert!(Solution::can_place_flowers(vec![1], 0));
    }

    #[test]
    fn fail2() {
        assert!(Solution::can_place_flowers(vec![0, 0, 1, 0, 0], 1));
    }
}
