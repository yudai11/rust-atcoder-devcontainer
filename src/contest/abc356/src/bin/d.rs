use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut where_m_is_pop: Vec<usize> = vec![];

    let mut ans: usize = 0;

    for i in 0..61 {
        let m = (m >> i) & 1;
        if m != 0 {
            where_m_is_pop.push(i);
        }
    }

    for &i in where_m_is_pop.iter() {
        ans += count(n, i);
        ans %= MOD;
    }

    println!("{}", ans);
}

fn count(n: usize, i: usize) -> usize {
    // let x = (1 << (i as u32 + 1)) as usize;
    let x = 2_usize.pow(i as u32 + 1);
    let t = n / x;
    let mut res: usize = t * (x / 2);
    res %= MOD;
    let y = n % x;
    if y >= x / 2 {
        res += y + 1 - x / 2;
        res %= MOD;
    }
    res
}
