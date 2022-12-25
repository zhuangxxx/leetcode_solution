struct Solution;

impl Solution {
    /// [925. 长按键入](https://leetcode.cn/problems/long-pressed-name/)
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let (mut n, mut t) = (0, 0);
        while n < name.len() && t < typed.len() {
            if name.as_bytes()[n] != typed.as_bytes()[t] {
                if n > 0 && name.as_bytes()[n - 1] == typed.as_bytes()[t] {
                    t += 1;
                    continue;
                }

                return false;
            }

            n += 1;
            t += 1;
        }

        while t < typed.len() {
            if name.as_bytes()[n - 1] != typed.as_bytes()[t] {
                return false;
            }

            t += 1;
        }

        n == name.len()
        // 0ms/2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_long_pressed_name(
            String::from("alex"),
            String::from("aaleex")
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_long_pressed_name(
            String::from("saeed"),
            String::from("ssaaedd")
        ));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::is_long_pressed_name(
            String::from("alex"),
            String::from("aaleexa")
        ));
    }

    #[test]
    fn fail2() {
        assert!(!Solution::is_long_pressed_name(
            String::from("vtkgn"),
            String::from("vttkgnn")
        ));
    }
}
