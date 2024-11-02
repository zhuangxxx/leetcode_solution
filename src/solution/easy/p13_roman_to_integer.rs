struct Solution;

impl Solution {
    /// [13. 罗马数字转整数](https://leetcode-cn.com/problems/roman-to-integer/)
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let vc = s.chars().collect::<Vec<char>>();
        let mut m = std::collections::HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        for i in 0..vc.len() - 1 {
            m.get(&vc[i]).map(|n| {
                if m.get(&vc[i + 1]).map(|next| next > n).unwrap_or(false) {
                    result -= n;
                } else {
                    result += n;
                }
            });
        }
        result += m.get(vc.last().unwrap()).unwrap_or(&0);
        result
        // 0ms/2MB
    }

    // #[inline]
    // fn get_int(c: &char) -> i32 {
    //     match c {
    //         'I' => 1,
    //         'V' => 5,
    //         'X' => 10,
    //         'L' => 50,
    //         'C' => 100,
    //         'D' => 500,
    //         'M' => 1000,
    //         _ => 0,
    //     }
    // }

    // pub fn roman_to_int(s: String) -> i32 {
    //     let mut result = 0;
    //     let c = s.chars().collect::<Vec<char>>();
    //     for i in 0..c.len() - 1 {
    //         let n = Self::get_int(c.get(i).unwrap_or(&' '));
    //         let next = Self::get_int(c.get(i + 1).unwrap_or(&' '));
    //         if next > n {
    //             result -= n;
    //         } else {
    //             result += n;
    //         }
    //     }
    //     result += Self::get_int(c.last().unwrap_or(&' '));
    //     result
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
