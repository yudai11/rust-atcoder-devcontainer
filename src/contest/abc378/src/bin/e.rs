use ac_library::{Additive, Segtree};
use proconio::input;
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
    const MAX: usize = 2 * 100001;

    input! {
        n: usize, m: usize,
        a: [usize; n]
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = (s[i] + a[i]) % m;
    }

    let mut ans = s.iter().sum::<usize>();

    let mut cnt_segtree = Segtree::<Additive<usize>>::new(MAX);
    let mut num_segtree = Segtree::<Additive<usize>>::new(MAX);

    cnt_segtree.set(s[1], 1);
    num_segtree.set(s[1], s[1]);

    for i in 2..=n {
        let x = s[i];
        ans += cnt_segtree.prod(0..x) * x - num_segtree.prod(0..x);
        ans += cnt_segtree.prod(x + 1..MAX) * (x + m) - num_segtree.prod(x + 1..MAX);

        cnt_segtree.set(x, cnt_segtree.get(x) + 1);
        num_segtree.set(x, num_segtree.get(x) + x);
    }

    println!("{}", ans);
}
