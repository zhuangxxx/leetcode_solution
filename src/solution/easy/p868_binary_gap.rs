struct Solution;

impl Solution {
    /// [868. 二进制间距](https://leetcode.cn/problems/binary-gap/)
    pub fn binary_gap(mut n: i32) -> i32 {
        if n.count_ones() < 2 {
            return 0;
        }

        let (mut s, mut p, mut m) = (None, 0, 0);
        while n > 0 {
            if n & 1 == 1 {
                if let Some(s) = s {
                    m = std::cmp::max(m, p - s);
                }
                s = Some(p);
            }
            p += 1;
            n >>= 1;
        }

        m
        // 0ms/2.1MB
    }

    // pub fn binary_gap(mut n: i32) -> i32 {
    //     format!("{:b}", n)
    //         .char_indices()
    //         .filter(|(_, i)| i == &'1')
    //         .map(|(i, _)| i)
    //         .fold((0, None), |(x, y), curr| {
    //             if let Some(m) = y {
    //                 return (std::cmp::max(x, curr - m), Some(curr));
    //             } else {
    //                 (0, Some(curr))
    //             }
    //         })
    //         .0 as i32
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::binary_gap(22), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::binary_gap(8), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::binary_gap(5), 2);
    }
}
