struct Solution;

impl Solution {
    /// [661. 图片平滑器](https://leetcode.cn/problems/image-smoother/)
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (img[0].len(), img.len());
        let mut s = vec![vec![0; m]; n];

        for x in 0..m {
            for y in 0..n {
                let mut c = 0;
                for i in x.saturating_sub(1)..=std::cmp::min(x + 1, m - 1) {
                    for j in y.saturating_sub(1)..=std::cmp::min(y + 1, n - 1) {
                        s[y][x] += img[j][i];
                        c += 1;
                    }
                }

                s[y][x] /= c;
            }
        }

        s
        // 16ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }
}
