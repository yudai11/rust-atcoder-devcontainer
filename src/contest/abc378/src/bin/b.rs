use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        qr: [(usize, usize); n],
        _q: usize,
        td: [(Usize1,usize); _q]
    }

    for &(t, d) in td.iter() {
        let mut ans = d;
        let (q, r) = qr[t];
        if d % q < r {
            ans += r - d % q;
        }
        if d % q > r {
            ans += q + r - d % q;
        }

        println!("{ans}");
    }
}
