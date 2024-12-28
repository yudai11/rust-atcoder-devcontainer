use std::collections::HashMap;

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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    input! {
        n: usize, t: usize,
        ab: [(Usize1,usize); t]
    }

    let mut score_kinds: HashMap<usize, usize> = HashMap::new();
    let mut scores: Vec<usize> = vec![0; n];
    score_kinds.insert(0, n);
    let mut ans: usize = 1;

    for i in 0..t {
        let ai = ab[i].0;
        let bi = ab[i].1;

        // old scoreのdelete
        let new_kind = score_kinds[&scores[ai]] - 1;
        score_kinds.remove(&scores[ai]);
        if new_kind == 0 {
            ans -= 1;
        } else {
            score_kinds.insert(scores[ai], new_kind);
        }

        // new scoreの追加
        let new_score = scores[ai] + bi;
        scores[ai] += bi;
        if !score_kinds.contains_key(&new_score) {
            score_kinds.insert(new_score, 1);
            ans += 1;
        } else {
            let new_kind = score_kinds[&new_score] + 1;
            score_kinds.remove(&new_score);
            score_kinds.insert(new_score, new_kind);
        }

        println!("{ans}");
    }
}
