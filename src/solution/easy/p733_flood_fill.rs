struct Solution;

impl Solution {
    /// [733. 图像渲染](https://leetcode.cn/problems/flood-fill/)
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        if image[sr as usize][sc as usize] == color {
            return image;
        }

        let old_color = image[sr as usize][sc as usize];

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((sr as usize, sc as usize));
        while let Some((r, c)) = queue.pop_front() {
            image[r][c] = color;
            if r > 0 && image[r - 1][c] == old_color {
                queue.push_back((r - 1, c));
            }
            if r < image.len() - 1 && image[r + 1][c] == old_color {
                queue.push_back((r + 1, c));
            }
            if c > 0 && image[r][c - 1] == old_color {
                queue.push_back((r, c - 1));
            }
            if c < image[0].len() - 1 && image[r][c + 1] == old_color {
                queue.push_back((r, c + 1));
            }
        }

        image
        // 0ms/2.3MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
