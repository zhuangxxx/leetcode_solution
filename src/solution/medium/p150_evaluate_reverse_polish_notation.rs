struct Solution;

impl Solution {
    /// [150. 逆波兰表达式求值](https://leetcode.cn/problems/evaluate-reverse-polish-notation/)
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let (mut stack, mut index) = (vec![0; (tokens.len() + 1) / 2 + 1], 0);
        for token in tokens {
            match token.as_str() {
                "+" => {
                    index -= 1;
                    stack[index] += stack[index + 1];
                }
                "-" => {
                    index -= 1;
                    stack[index] -= stack[index + 1];
                }
                "*" => {
                    index -= 1;
                    stack[index] *= stack[index + 1];
                }
                "/" => {
                    index -= 1;
                    stack[index] /= stack[index + 1];
                }
                _ => {
                    index += 1;
                    stack[index] = token.parse().unwrap();
                }
            }
        }

        stack[index]
        // 2ms/2.70MB
    }

    // pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    //     #[inline]
    //     fn top(stack: &mut Vec<i32>) -> (i32, i32) {
    //         (stack.pop().unwrap(), stack.pop().unwrap())
    //     }
    //     let mut stack = Vec::new();
    //     for token in tokens {
    //         match token.as_str() {
    //             "+" => {
    //                 let (a, b) = top(&mut stack);
    //                 stack.push(b + a);
    //             }
    //             "-" => {
    //                 let (a, b) = top(&mut stack);
    //                 stack.push(b - a);
    //             }
    //             "*" => {
    //                 let (a, b) = top(&mut stack);
    //                 stack.push(b * a);
    //             }
    //             "/" => {
    //                 let (a, b) = top(&mut stack);
    //                 stack.push(b / a);
    //             }
    //             _ => stack.push(token.parse::<i32>().unwrap()),
    //         }
    //     }

    //     *stack.last().unwrap()
    //     // 2ms/2.72MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["2", "1", "+", "3", "*"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            9
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["4", "13", "5", "/", "+"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            6
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            ),
            22
        );
    }
}
