// use num::pow;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: u128, k: usize
    }

    let mut ans = n;
    for _ in 0..k {
        ans = num_transform(ans.clone());
    }

    println!("{}", ans);
}

fn num_transform(n: u128) -> u128 {
    let mut n_10 = 0;
    let mut cp_n = n.clone();
    let mut pow_index = 1;
    for _i in 0..20 {
        n_10 += cp_n % 10 * pow_index;
        pow_index *= 8;
        cp_n /= 10;
        if cp_n == 0 {
            break;
        }
    }
    // println!("{n_10}");

    let mut n_list = [0; 20];
    let mut cp_n_10 = n_10;
    for i in 0..20 {
        n_list[i] = cp_n_10 % 9;
        cp_n_10 /= 9;
        if cp_n_10 == 0 {
            break;
        }
    }
    // println!("{:?}", n_list);

    let mut ans = 0;
    let mut pow_index = 1;
    for i in 0..20 {
        if n_list[i] == 8 {
            n_list[i] = 5;
        }
        ans += n_list[i] * pow_index;
        pow_index *= 10;
    }

    return ans;
    // println!("{ans}");

    // let mut ans_8 = 0;
    // let mut cp_ans_10 = ans_10;
    // let mut pow_index = 1;
    // for _i in 0..20 {
    //     ans_8 += cp_ans_10 % 8 * pow_index;
    //     pow_index *= 10;
    //     cp_ans_10 /= 8;
    // }
    // ans_8
}
