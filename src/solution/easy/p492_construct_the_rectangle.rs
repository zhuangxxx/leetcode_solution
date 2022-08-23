struct Solution;

impl Solution {
    /// [492. 构造矩形](https://leetcode.cn/problems/construct-the-rectangle/)
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = {
            let (mut i, mut j) = (1i64, area as i64 / 2);
            while i < j {
                let m = j - (j - i) / 2;
                let p = m * m;
                if p > area as i64 {
                    j = m - 1;
                } else if p < area as i64 {
                    i = m;
                } else {
                    i = m;
                    j = m;
                    break;
                }
            }
            i
        } as i32;

        while w > 0 {
            if area % w == 0 {
                break;
            }

            w -= 1;
        }

        vec![area / w, w]
        // 0ms/2MB
    }

    // pub fn construct_rectangle(area: i32) -> Vec<i32> {
    //     let mut w = (area as f32).sqrt().round() as i32;
    //     while w > 0 {
    //         if area % w == 0 {
    //             break;
    //         }

    //         w -= 1;
    //     }

    //     vec![area / w, w]
    //     // 0ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::construct_rectangle(10000000), vec![3200, 3125]);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::construct_rectangle(1), vec![1, 1]);
    }
}
