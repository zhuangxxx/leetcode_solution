struct Solution;

impl Solution {
    /// [3233. 统计不是特殊数字的数字数量](https://leetcode.cn/problems/find-the-count-of-numbers-which-are-not-special/)
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let n = (r as f32).sqrt() as usize;

        let mut cnt = r - l + 1;
        let mut visit = vec![false; n + 1];
        for i in 2..=n {
            if !visit[i] {
                let s = (i * i) as i32;
                if s >= l && s <= r {
                    cnt -= 1;
                }
                for j in (i * 2..=n).step_by(i) {
                    visit[j] = true;
                }
            }
        }

        cnt
        // 23ms/2.30MB
    }

    // pub fn non_special_count(l: i32, r: i32) -> i32 {
    //     #[inline]
    //     fn is_prime(x: i32) -> bool {
    //         if x == 1 {
    //             false
    //         } else if x == 2 || x == 3 {
    //             true
    //         } else if x % 6 != 1 && x % 6 != 5 {
    //             false
    //         } else {
    //             for y in (5..=(x as f32).sqrt() as i32).step_by(6) {
    //                 if x % y == 0 || x % (y + 2) == 0 {
    //                     return false;
    //                 }
    //             }

    //             true
    //         }
    //     }

    //     let mut cnt = 0;
    //     for x in (l as f32).sqrt().ceil() as i32..=(r as f32).sqrt() as i32 {
    //         if is_prime(x) {
    //             cnt += 1;
    //         }
    //     }

    //     r - l + 1 - cnt
    //     // 18ms/2.41MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::non_special_count(5, 7), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::non_special_count(4, 16), 11);
    }
}
