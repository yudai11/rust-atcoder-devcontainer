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
// use ac_library::{Additive, Segtree}; // segtree

fn main() {
    struct Person {
        name: String,
        score: usize,
    }

    input! {
        points: [usize;5]
    }

    // let mut ans: [(&str, usize); 32] = todo!();
    let mut names = vec![];
    let mut scores = vec![];
    let chars = ['A', 'B', 'C', 'D', 'E'];

    for i in 0..(1 << 5) as usize {
        let mut name = String::from("");
        let mut cum_sum: usize = 0;

        for j in 0..5 {
            if (i & (1 << j)) != 0 {
                name.push(chars[j]);
                cum_sum += points[j];
            }
        }
        names.push(name);
        scores.push(cum_sum);
    }

    let mut ans = vec![];

    for i in 1..32 {
        ans.push((names[i].clone(), scores[i]));
    }

    ans.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));

    for i in 0..31 {
        println!("{}", ans[i].0);
    }
}
