use std::collections::HashMap;

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
        n: usize,
        a: [usize; n]
    }

    let mut emerge_x: HashMap<usize, usize> = HashMap::new();
    let mut b: Vec<isize> = vec![-1; n];

    for i in 0..n {
        let ai = a[i];
        if emerge_x.contains_key(&ai) {
            b[i] = emerge_x[&ai] as isize;
            emerge_x.remove(&ai);
        }

        emerge_x.insert(ai, i + 1);
    }

    for &b in b.iter() {
        print!("{b} ");
    }
}
