// use num::pow;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [[usize;6];n]
    }

    let mut sum_of_row = vec![0; n];
    for i in 0..n {
        sum_of_row[i] = a[i].iter().sum::<usize>() as u64
    }
    let mut ans: u64 = 1;
    for i in 0..n {
        ans = ans * sum_of_row[i];
        ans %= 1000000007;
    }

    println!("{:?}", ans);
}

fn pow_mine(c: usize, b: usize) -> usize {
    let n = b.next_power_of_two();
    let mut b_bi_list: Vec<bool> = vec![false; n];
    for i in 0..n {
        if (b >> i) & 1 == 1 {
            b_bi_list[i] = true;
        }
    }
    let mut ans: usize = 1;
    let mut c_pow_2i: usize = c;

    for i in 0..n {
        ans *= if b_bi_list[i] { c_pow_2i } else { 1 };
        c_pow_2i *= c_pow_2i;
    }

    ans
}
