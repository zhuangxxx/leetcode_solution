struct Solution;

impl Solution {
    /// [821. 字符的最短距离](https://leetcode.cn/problems/shortest-distance-to-a-character/)
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut d = vec![0; s.len()];
        let (s, l) = (s.chars().collect::<Vec<_>>(), s.len());

        let mut n = -(l as i32);
        for i in 0..l {
            if s[i] == c {
                n = i as i32;
            }

            d[i] = i as i32 - n;
        }

        n = 2 * (l as i32);
        for i in (0..l).rev() {
            if s[i] == c {
                n = i as i32;
            }

            d[i] = std::cmp::min(d[i], n - i as i32);
        }

        d
        // 0ms/2.2MB
    }

    // pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    //     let mut d = vec![0; s.len()];
    //     let mut p = 0;
    //     for (i, sc) in s.char_indices() {
    //         if sc == c {
    //             if p == 0 {
    //                 for j in p..i {
    //                     d[j] = (i - j) as i32;
    //                 }
    //             } else {
    //                 let m = p + (i - p) / 2;
    //                 for j in p..m {
    //                     d[j] = (j - p + 1) as i32;
    //                 }
    //                 for j in m..i {
    //                     d[j] = (i - j) as i32;
    //                 }
    //             }

    //             p = i + 1;
    //         } else if i == s.len() - 1 {
    //             for j in p..=i {
    //                 d[j] = (j - p + 1) as i32;
    //             }
    //         }
    //     }

    //     d
    //     // 0ms/1.9MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::shortest_to_char("aaab".to_string(), 'b'),
            vec![3, 2, 1, 0]
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::shortest_to_char("love leet code".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 1, 0, 0, 1, 2, 3, 2, 1, 0]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::shortest_to_char("aaba".to_string(), 'b'),
            vec![2, 1, 0, 1]
        );
    }
}
