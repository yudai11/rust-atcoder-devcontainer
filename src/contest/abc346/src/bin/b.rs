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
        w: usize, b: usize,
    }

    let _t = "wbwbwwbwbwbw";
    let mut t = vec![];
    for x in _t.chars() {
        t.push(x);
    }

    for i in 0..12 {
        let mut num_b: usize = 0;
        let mut num_w: usize = 0;
        for j in 0..w + b {
            if t[(i + j) % 12] == 'b' {
                num_b += 1;
            } else {
                num_w += 1;
            }
        }

        if num_b == b && num_w == w {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
