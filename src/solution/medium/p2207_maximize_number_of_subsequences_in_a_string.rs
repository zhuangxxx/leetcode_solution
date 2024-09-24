struct Solution;

impl Solution {
    /// [2207. 字符串中最多数目的子序列](https://leetcode.cn/problems/maximize-number-of-subsequences-in-a-string/)
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let pattern = pattern.as_bytes();
        let mut r = 0;
        let (mut c1, mut c2) = (0, 0);
        for b in text.bytes() {
            if b == pattern[1] {
                r += c1;
                c2 += 1;
            }
            if b == pattern[0] {
                c1 += 1;
            }
        }

        r + std::cmp::max(c1, c2)
        // 0ms/2.40MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximum_subsequence_count("abdcdbc".to_string(), String::from("ac")),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximum_subsequence_count("aabb".to_string(), String::from("ab")),
            6
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::maximum_subsequence_count("zigfj".to_string(), String::from("ju")),
            1
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(
            Solution::maximum_subsequence_count(
                "iuvgbmteyivbfwuospxmmgzagfa".to_string(),
                String::from("ti")
            ),
            3
        );
    }

    #[test]
    fn fail3() {
        assert_eq!(
            Solution::maximum_subsequence_count(
                "mzyzmhrbgfvtkryzpqoacbwtdpri".to_string(),
                String::from("ty")
            ),
            3
        );
    }

    #[test]
    fn fail4() {
        assert_eq!(
            Solution::maximum_subsequence_count(
                "vnedkpkkyxelxqptfwuzcjhqmwagvrglkeivowvbjdoyydnjrqrqejoyptzoklaxcjxbrrfmpdxckfjzahparhpanwqfjrpbslsyiwbldnpjqishlsuagevjmiyktgofvnyncizswldwnngnkifmaxbmospdeslxirofgqouaapfgltgqxdhurxljcepdpndqqgfwkfiqrwuwxfamciyweehktaegynfumwnhrgrhcluenpnoieqdivznrjljcotysnlylyswvdlkgsvrotavnkifwmnvgagjykxgwaimavqsxuitknmbxppgzfwtjdvegapcplreokicxcsbdrsyfpustpxxssnouifkypwqrywprjlyddrggkcglbgcrbihgpxxosmejchmzkydhquevpschkpyulqxgduqkqgwnsowxrmgqbmltrltzqmmpjilpfxocflpkwithsjlljxdygfvstvwqsyxlkknmgpppupgjvfgmxnwmvrfuwcrsadomyddazlonjyjdeswwznkaeaasyvurpgyvjsiltiykwquesfjmuswjlrphsdthmuqkrhynmqnfqdlwnwesdmiiqvcpingbcgcsvqmsmskesrajqwmgtdoktreqssutpudfykriqhblntfabspbeddpdkownehqszbmddizdgtqmobirwbopmoqzwydnpqnvkwadajbecmajilzkfwjnpfyamudpppuxhlcngkign".to_string(),
                String::from("rr")
            ),
            496
        );
    }
}
