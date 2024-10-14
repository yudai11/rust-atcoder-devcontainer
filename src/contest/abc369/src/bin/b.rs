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
        _as : [(isize,char);n]
    }

    let mut l = vec![];
    let mut r = vec![];

    for i in 0..n {
        if _as[i].1 == 'L' {
            l.push(_as[i].0);
        } else {
            r.push(_as[i].0);
        }
    }

    let mut ans: isize = 0;

    if !l.is_empty() {
        let mut cur_li = l[0];
        for (i, li) in l.iter().enumerate() {
            ans += (cur_li - l[i]).abs();
            cur_li = l[i];
        }
    }
    if !r.is_empty() {
        let mut cur_ri = r[0];
        for (i, ri) in r.iter().enumerate() {
            ans += (cur_ri - r[i]).abs();
            cur_ri = r[i];
        }
    }

    println!("{}", ans);
}
