struct Solution;

impl Solution {
    /// [3200. 三角形的最大高度](https://leetcode.cn/problems/maximum-height-of-a-triangle/)
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        std::cmp::max(
            std::cmp::min(
                2 * ((red as f32).sqrt() as i32) - 1,
                2 * (((-1. + (1. + 4. * blue as f32).sqrt()) / 2.) as i32),
            ),
            std::cmp::min(
                2 * ((blue as f32).sqrt() as i32) - 1,
                2 * (((-1. + (1. + 4. * red as f32).sqrt()) / 2.) as i32),
            ),
        ) + 1
        // 3ms/2.04MB
    }

    // pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    //     #[inline]
    //     fn height(mut a: i32, mut b: i32) -> i32 {
    //         let mut height = 1;
    //         loop {
    //             if height & 1 == 1 {
    //                 a -= height;
    //             } else {
    //                 b -= height;
    //             }
    //             if a < 0 || b < 0 {
    //                 height -= 1;
    //                 break;
    //             }
    //             height += 1;
    //         }

    //         height
    //     }

    //     std::cmp::max(height(red, blue), height(blue, red))
    //     // 3ms/1.95MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
    }
}
