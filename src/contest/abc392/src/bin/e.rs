// use itertools::Itertools;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// 優先度付きのque, peek,popで最大値を散り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()で最小値を取れる)
// use proconio::marker::Isize1;
use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree
use ac_library::Dsu;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut not_used = vec![];
    for i in 0..m {
        let (a, b) = ab[i];
        if !dsu.same(a, b) {
            dsu.merge(a, b);
        } else {
            not_used.push(i);
        }
    }

    let mut leaders = HashSet::new();
    for i in 0..n {
        leaders.insert(dsu.leader(i));
    }

    let mut ans = vec![];
    for i in not_used {
        if leaders.len() == 1 {
            break;
        }

        let (a, _b) = ab[i];
        let base_leader = dsu.leader(a);
        leaders.remove(&base_leader);

        let connect_leader = *leaders.iter().next().unwrap(); // leadersの最初の要素のpointerの指す中身
        leaders.remove(&connect_leader);

        ans.push((i + 1, a + 1, connect_leader + 1));

        dsu.merge(a, connect_leader);
        leaders.insert(dsu.leader(a));
    }

    println!("{}", ans.len());
    for &(i, a, b) in &ans {
        println!("{} {} {}", i, a, b);
    }
}
