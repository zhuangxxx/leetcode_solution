pub struct Solution;

impl Solution {
    /// # 13. [罗马数字转整数](https://leetcode-cn.com/problems/roman-to-integer/)
    /// 罗马数字包含以下七种字符:`I`，`V`，`X`，`L`，`C`，`D` 和 `M`。
    /// ```
    /// 字符          数值
    /// I             1
    /// V             5
    /// X             10
    /// L             50
    /// C             100
    /// D             500
    /// M             1000
    /// ```
    /// 例如， 罗马数字`2`写做`II`，即为两个并列的 1。`12`写做`XII`，即为`X` + `II`。`27`写做`XXVII`，即为`XX` + `V` + `II`。
    /// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 `IIII`，而是 `IV`。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 `IX`。这个特殊的规则只适用于以下六种情况：
    /// - I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
    /// - X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
    /// - C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
    /// 给定一个罗马数字，将其转换成整数。
    /// 示例 1:
    /// ```
    /// 输入: "III"
    /// 输出: 3
    /// ```
    /// 示例 2:
    /// ```
    /// 输入: "IV"
    /// 输出: 4
    /// ```
    /// 示例 3:
    /// ```
    /// 输入: "IX"
    /// 输出: 9
    /// ```
    /// 示例 4:
    /// ```
    /// 输入: "LVIII"
    /// 输出: 58
    /// 解释: L = 50, V= 5, III = 3.
    /// ```
    /// 示例 5:
    /// ```
    /// 输入: "MCMXCIV"
    /// 输出: 1994
    /// 解释: M = 1000, CM = 900, XC = 90, IV = 4.
    /// ```
    /// 提示：
    /// - `0 <= s.length <= 15`
    /// - `s` 仅含字符 `('I', 'V', 'X', 'L', 'C', 'D', 'M')`
    /// - 题目数据保证 `s` 是一个有效的罗马数字，且表示整数在 `[1, 3999]` 范围内。
    /// - 题目所给测试用例皆符合罗马数字书写规则，不会出现跨位等情况。
    /// - IL 和 IM 这样的例子并不符合题目的要求，49 应该写作 XLIX，999 应该写作 CMXCIX。
    /// - 关于罗马数字的规则，可以参考 [罗马数字 - Mathematics ](https://b2b.partcommunity.com/community/knowledge/zh_CN/detail/10753/%E7%BD%97%E9%A9%AC%E6%95%B0%E5%AD%97#knowledge_article)。
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let vc = s.chars().collect::<Vec<char>>();
        let mut m = std::collections::HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        for i in 0..vc.len() - 1 {
            m.get(&vc[i]).map(|n| {
                if m.get(&vc[i + 1]).map(|next| next > n).unwrap_or(false) {
                    result -= n;
                } else {
                    result += n;
                }
            });
        }
        result += m.get(vc.last().unwrap()).unwrap_or(&0);
        result
        // 0ms/2MB
    }

    // #[inline]
    // fn get_int(c: &char) -> i32 {
    //     match c {
    //         'I' => 1,
    //         'V' => 5,
    //         'X' => 10,
    //         'L' => 50,
    //         'C' => 100,
    //         'D' => 500,
    //         'M' => 1000,
    //         _ => 0,
    //     }
    // }

    // pub fn roman_to_int(s: String) -> i32 {
    //     let mut result = 0;
    //     let c = s.chars().collect::<Vec<char>>();
    //     for i in 0..c.len() - 1 {
    //         let n = Self::get_int(c.get(i).unwrap_or(&' '));
    //         let next = Self::get_int(c.get(i + 1).unwrap_or(&' '));
    //         if next > n {
    //             result -= n;
    //         } else {
    //             result += n;
    //         }
    //     }
    //     result += Self::get_int(c.last().unwrap_or(&' '));
    //     result
    //     // 4ms/2.1MB
    // }
}