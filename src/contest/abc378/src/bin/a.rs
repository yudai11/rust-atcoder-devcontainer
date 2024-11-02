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
        a: [usize; 4]
    }

    let mut boals = [0; 5];
    for x in a {
        boals[x] += 1;
    }

    let mut ans = 0;

    for v in boals {
        if v >= 2 {
            ans += 1;
        }

        if v >= 4 {
            ans += 1;
        }
    }

    println!("{ans}");
}
