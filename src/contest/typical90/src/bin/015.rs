use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    const MOD: usize = 1000_000_007;
    let mut ans = vec![];

    let mut facts = vec![1_usize; n + 1];
    for i in 1..=n {
        facts[i] = facts[i - 1] * i;
        facts[i] %= MOD;
    }

    let mut facts_inv = vec![1_usize; n + 1];
    for i in 2..=n {
        facts_inv[i] = mod_inv(facts[i], MOD);
    }

    for k in 1..=n {
        let mut res = 0_usize;
        for a in 1..=n {
            if a + (k - 1) * (a - 1) > n {
                break;
            }
            let m = n - (k - 1) * (a - 1);
            res += facts[m] * facts_inv[a] % MOD * facts_inv[m - a] % MOD;
            res %= MOD;
        }
        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"));
}

// xの逆元を出力
fn mod_inv(x: usize, m: usize) -> usize {
    let x = x as isize;
    let m = m as isize;
    let mut a = vec![x, m];
    let mut b = vec![1_isize, 0_isize];
    while a[0] > 0 && a[1] > 0 {
        let t = a[0] / a[1];
        a[0] -= t * a[1];
        a.reverse();
        b[0] -= t * b[1];
        b.reverse();
    }
    b[0] %= m;
    if b[0] < 0 {
        b[0] += m;
    }
    return b[0] as usize;
}
