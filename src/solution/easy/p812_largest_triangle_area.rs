struct Solution;

impl Solution {
    /// [812. 最大三角形面积](https://leetcode.cn/problems/largest-triangle-area/)
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut max = 0.;
        for i in 0..points.len() {
            for j in 0..points.len() {
                for k in 0..points.len() {
                    let x0 = points[i][0];
                    let x1 = points[j][0] - x0;
                    let x2 = points[k][0] - x0;
                    let y0 = points[i][1];
                    let y1 = points[j][1] - y0;
                    let y2 = points[k][1] - y0;
                    let area = (x1 * y2 - x2 * y1).abs() as f64 / 2.;

                    if max < area {
                        max = area;
                    }
                }
            }
        }

        max
        // 4ms/2MB
    }

    // pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    //     let mut max = 0.;
    //     for i in 0..points.len() {
    //         for j in 0..points.len() {
    //             for k in 0..points.len() {
    //                 let (x0, y0, x1, y1, x2, y2) = (
    //                     points[i][0] as f64,
    //                     points[i][1] as f64,
    //                     points[j][0] as f64,
    //                     points[j][1] as f64,
    //                     points[k][0] as f64,
    //                     points[k][1] as f64,
    //                 );
    //                 let area =
    //                     0.5 * (x0 * y1 + x1 * y2 + x2 * y0 - x0 * y2 - x1 * y0 - x2 * y1).abs();

    //                 if max < area {
    //                     max = area;
    //                 }
    //             }
    //         }
    //     }

    //     max
    //     // 4ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(
            (Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]) - 2.)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn fail1() {
        assert!(
            (Solution::largest_triangle_area(vec![vec![4, 6], vec![6, 5], vec![3, 1],]) - 5.5)
                .abs()
                < f64::EPSILON
        );
    }
}
