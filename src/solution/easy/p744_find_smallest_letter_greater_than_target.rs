struct Solution;

impl Solution {
    /// [744. 寻找比目标字母大的最小字母](https://leetcode.cn/problems/find-smallest-letter-greater-than-target/)
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        if letters.is_empty() {
            target
        } else if target >= letters[letters.len() - 1] {
            letters[0]
        } else {
            let (mut l, mut r) = (0, letters.len() - 1);
            while l < r {
                let m = (r - l) / 2 + l;
                if letters[m] > target {
                    r = m;
                } else {
                    l = m + 1;
                }
            }

            letters[l]
        }
        // 4ms/2.7MB
    }

    // pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    //     if letters.len() == 0 {
    //         target
    //     } else if target >= letters[letters.len() - 1] {
    //         letters[0]
    //     } else {
    //         for letter in letters {
    //             if letter > target {
    //                 return letter;
    //             }
    //         }

    //         target
    //     }
    //     // 4ms/2.6MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'),
            'c'
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'),
            'f'
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'),
            'f'
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'z'),
            'c'
        );
    }
}
