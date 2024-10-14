use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize,
        _a: [usize; n],
    }

    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = _a[i];
    }

    let mut ans = 0;
    loop {
        a.sort();
        if a[n - 2] == 0 {
            break;
        }
        a[n - 2] -= 1;
        a[n - 1] -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
