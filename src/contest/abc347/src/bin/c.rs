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
    input! {
        n: usize, a: usize, b: usize,
        d: [usize; n]
    }

    let mut d_mod = vec![0; 2 * n];
    for i in 0..n {
        let x = d[i] % (a + b);
        d_mod[i] = x;
        d_mod[i + n] = x + a + b;
    }

    d_mod.sort();

    let mut cur_day = d_mod[0];
    for i in 1..2 * n {
        if d_mod[i] - cur_day > b {
            println!("Yes");
            return;
        } else {
            cur_day = d_mod[i];
        }
    }

    println!("No");
}
