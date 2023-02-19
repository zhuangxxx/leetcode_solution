struct Solution;

impl Solution {
    /// [7. 整数反转](https://leetcode.cn/problems/reverse-integer/)
    pub fn reverse(mut x: i32) -> i32 {
        const MAX_DIV_10: i32 = i32::MAX / 10;
        let n = x < 0;

        let mut r = 0;
        while x != 0 {
            if r > MAX_DIV_10 {
                return 0;
            }

            r = r * 10 + (x % 10).abs();
            x /= 10;
        }

        r * if n { -1 } else { 1 }
        // 0ms/1.9MB
    }

    // pub fn reverse(mut x: i32) -> i32 {
    //     const MAX: [i32; 10] = [2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
    //     let n = x < 0;

    //     let mut r = Vec::new();
    //     while x != 0 {
    //         r.push((x % 10).abs());
    //         x /= 10;
    //     }

    //     if r.len() == MAX.len() {
    //         for i in 0..r.len() {
    //             if r[i] > MAX[i] {
    //                 return 0;
    //             } else if r[i] == MAX[i] {
    //                 continue;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }

    //     r.into_iter().fold(0, |p, c| p * 10 + c) * if n { -1 } else { 1 }
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::reverse(i32::MAX), 0);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::reverse(-2147483641), -1463847412);
    }
}
