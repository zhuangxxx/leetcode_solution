struct Solution;

impl Solution {
    /// [888. 公平的糖果交换](https://leetcode.cn/problems/fair-candy-swap/)
    pub fn fair_candy_swap(mut alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        alice_sizes.sort_unstable();

        let (diff, len) = (
            (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>()) / 2,
            alice_sizes.len() - 1,
        );
        for bob in bob_sizes {
            let alice = diff + bob;

            if alice_sizes[0] > alice || alice_sizes[len] < alice {
                continue;
            }

            let (mut l, mut r) = (0, len);
            while l < r {
                let m = l + (r - l) / 2;
                if alice_sizes[m] >= alice {
                    r = m;
                } else if alice_sizes[m] < alice {
                    l = m + 1;
                }
            }

            if alice_sizes[l] == alice {
                return vec![alice, bob];
            }
        }

        vec![0, 0]
        // 12ms/2.2MB
    }

    // pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    //     let diff = (alice_sizes.iter().sum::<i32>() - bob_sizes.iter().sum::<i32>()) / 2;
    //     let set = alice_sizes.iter().collect::<std::collections::HashSet<_>>();
    //     for bob in bob_sizes {
    //         let alice = diff + bob;
    //         if set.contains(&alice) {
    //             return vec![alice, bob];
    //         }
    //     }

    //     vec![0, 0]
    //     // 16ms/2.4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
            vec![1, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2], vec![2, 3]),
            vec![1, 2]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]),
            vec![5, 4]
        );
    }
}
