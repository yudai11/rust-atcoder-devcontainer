use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    const MOD: usize = 998244353;

    let mut next_10_pow_list = vec![0; n];
    for i in 0..n {
        let &x = &a[i];
        next_10_pow_list[i] = find_next_10_pow(x);
    }

    let mut cum_sum_n10 = vec![0; n + 1];
    for i in 0..n {
        cum_sum_n10[i + 1] = cum_sum_n10[i] + next_10_pow_list[i];
        cum_sum_n10[i + 1] %= MOD;
    }

    let mut ans = 0;
    for i in 1..=n {
        // 下桁に現れるとき
        // ans += a[i - 1] * ();
        // 上桁に現れるとき
        ans += a[i - 1] * (i - 1 + cum_sum_n10[n] + MOD - cum_sum_n10[i]);
        ans %= MOD;
    }
    println!("{ans}");
}

fn find_next_10_pow(x: usize) -> usize {
    let mut y = 1;
    while y <= x {
        y *= 10;
    }
    return y;
}
