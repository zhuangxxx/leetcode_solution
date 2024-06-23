struct Solution;

impl Solution {
    /// [763. 划分字母区间](https://leetcode.cn/problems/partition-labels/)
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];
        for (i, b) in s.bytes().enumerate() {
            last[(b - b'a') as usize] = i;
        }

        let mut partition = Vec::new();
        let (mut start, mut end) = (0, 0);
        for (i, b) in s.bytes().enumerate() {
            end = std::cmp::max(end, last[(b - b'a') as usize]);
            if i == end {
                partition.push((end - start) as i32 + 1);
                start = end + 1;
            }
        }

        partition
        // 0ms/2.18MB
    }

    // pub fn partition_labels(s: String) -> Vec<i32> {
    //     let mut map = [[1, 0]; 26];
    //     for (i, b) in s.bytes().enumerate() {
    //         let a = (b - b'a') as usize;
    //         if map[a][0] > map[a][1] {
    //             map[a][0] = i;
    //         }
    //         map[a][1] = i;
    //     }

    //     let mut partitions = map.into_iter().filter(|i| i[0] <= i[1]).collect::<Vec<_>>();
    //     partitions.sort_unstable();

    //     let mut result = Vec::new();

    //     let mut interval = partitions[0];
    //     let (mut i, mut j) = (0, 1);
    //     while j < partitions.len() {
    //         if interval[1] > partitions[j][0] {
    //             interval[1] = std::cmp::max(interval[1], partitions[j][1]);
    //         } else {
    //             result.push((interval[1] - interval[0]) as i32 + 1);
    //             interval = partitions[j];
    //             i = j;
    //         }
    //         j += 1;
    //     }
    //     if i != j {
    //         result.push((interval[1] - interval[0]) as i32 + 1);
    //     }

    //     result
    //     // 0ms/2.11MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
    }
}
