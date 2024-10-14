use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: u128, k: u128
    }

    const MOD: u128 = 1000000007;
    if n == 1 {
        println!("{}", k);
    } else if n == 2 {
        if k < n {
            println!("0");
        } else {
            println!("{}", k * (k - 1) % MOD);
        }
    } else {
        if k < 3 {
            println!("0");
        } else {
            let mut ans: u128 = k * (k - 1) % MOD;
            ans *= pow_mine(k - 2, ((n - 2) % (MOD - 1)));
            ans %= MOD;
            println!("{}", ans);
        }
    }
}

fn pow_mine(c: u128, b: u128) -> u128 {
    const MOD: u128 = 1000000007;
    let mut n = 1;
    let mut x: u128 = 1;
    while x < b {
        n += 1;
        x *= 2;
    }
    let mut b_bi_list: Vec<bool> = vec![false; n];
    for i in 0..n {
        if (b >> i) & 1 == 1 {
            b_bi_list[i] = true;
        }
    }
    let mut ans: u128 = 1;
    let mut c_pow_2i: u128 = c;

    for i in 0..n {
        ans *= if b_bi_list[i] { c_pow_2i } else { 1 };
        ans %= MOD;
        c_pow_2i *= c_pow_2i;
        c_pow_2i %= MOD;
    }

    ans
}
