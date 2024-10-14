use itertools::Itertools;
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
        n: usize, s: f64, t: f64,
        abcd: [(f64, f64, f64, f64);n]
    }

    let mut ans = f64::MAX;

    // 印字間の距離
    for perm in (0..n).permutations(n) {
        for mask in 0..(1 << n) {
            let mut cur = (0.0, 0.0);
            let mut total = 0.0;
            for i in 0..n {
                let (a, b, c, d) = abcd[perm[i]];
                if ((mask >> i) & 1) == 0 {
                    total += ((cur.0 - a) * (cur.0 - a) + (cur.1 - b) * (cur.1 - b)).sqrt() / s;
                    cur = (c, d);
                } else {
                    total += ((cur.0 - c) * (cur.0 - c) + (cur.1 - d) * (cur.1 - d)).sqrt() / s;
                    cur = (a, b);
                }
            }

            ans = ans.min(total);
        }
    }

    // 印字の距離
    for i in 0..n {
        let (a, b, c, d) = abcd[i];
        let dist = ((a - c) * (a - c) + (b - d) * (b - d)).sqrt() / t;
        ans += dist;
    }
    println!("{ans}");
}
