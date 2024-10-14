use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut ans: u64 = (2 * n - 1) as u64;

    let mut i = 0;
    // 長さ3以上の列を捜査
    while i + 2 < n {
        let mut m: i64 = 2;
        let res = a[i + 1] - a[i];
        let mut j = i + 1;
        while j + 1 < n {
            if a[j + 1] - a[j] != res {
                break;
            }
            m += 1;
            j += 1;
        }

        if m > 2 {
            ans += ((m - 1) * (m - 2) / 2) as u64;
        }

        i = j;
    }

    println!("{}", ans);
}
