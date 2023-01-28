struct Solution;

impl Solution {
    /// [989. 数组形式的整数加法](https://leetcode.cn/problems/add-to-array-form-of-integer/)
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut add = Vec::new();
        while k > 0 {
            add.push(k % 10);
            k /= 10;
        }
        num.reverse();
        if num.len() < add.len() {
            num.resize(add.len(), 0);
        }

        let mut one = false;
        for i in 0..num.len() {
            let s = num[i] + if i < add.len() { add[i] } else { 0 } + if one { 1 } else { 0 };
            if s > 9 {
                num[i] = s % 10;
                one = true;
            } else {
                num[i] = s;
                one = false;
            }
        }
        if one {
            num.push(1);
        }

        num.reverse();
        num
        // 8ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 7, 4], 181),
            vec![4, 5, 5]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::add_to_array_form(vec![2, 1, 5], 806),
            vec![1, 0, 2, 1]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
