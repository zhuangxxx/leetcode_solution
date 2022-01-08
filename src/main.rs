#[allow(unused)]
mod leet_code;

fn main() {
    let i = leet_code::p13_roman_to_integer::Solution::roman_to_int("MCMXCIV".to_string());
    println!("{:?}", i);
}
