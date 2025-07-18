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
        n: usize, m: usize,
        lr: [(Usize1,Usize1);n]
    }

    // 第2成分(ri)でソート
    // lr.sort_by_key(|&(_, x)| x);

    // r_i=x となるiに対してmax(l_i)を返す
    let mut max_l: Vec<usize> = vec![0; m];
    for &(li, ri) in lr.iter() {
        max_l[ri] = max_l[ri].max(li + 1);
    }

    let mut ans: usize = 0;

    let mut left: usize = 0;
    // let mut right = 0;
    for right in 0..m {
        left = left.max(max_l[right]);
        ans += right + 1 - left;
    }

    println!("{ans}");
}
