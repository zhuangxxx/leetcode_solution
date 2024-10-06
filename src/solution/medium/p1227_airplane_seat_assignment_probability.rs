struct Solution;

impl Solution {
    /// [1227. 飞机座位分配概率](https://leetcode.cn/problems/airplane-seat-assignment-probability/)
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.
        } else {
            0.5
        }
        // 0ms/2.13MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::nth_person_gets_nth_seat(1), 1.);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::nth_person_gets_nth_seat(2), 0.5);
    }
}
