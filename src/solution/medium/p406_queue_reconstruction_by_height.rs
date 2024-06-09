struct Solution;

impl Solution {
    /// [406. 根据身高重建队列](https://leetcode.cn/problems/queue-reconstruction-by-height/)
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0]).reverse()
            }
        });

        let mut queue = Vec::with_capacity(people.len());
        for p in people {
            if (p[1] as usize) < queue.len() {
                queue.insert(p[1] as usize, p.clone());
            } else {
                queue.push(p.clone());
            }
        }

        queue
        // 3ms/2.28MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4]
            ]),
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ]
        );
    }
}
