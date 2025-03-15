use num_integer::Roots;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;
// use ac_library::{Additive, Segtree}; // segtree,isizeで使う.
// use ac_library::Dsu;
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: u128
    }

    // z := x - y, w := xy とすると, z (z^2 + 3 w) == n　and (t^2 + zt - w = 0 が解を持つ　<=> z^2 + 4w >= 0)

    for i in 1..=1000_000 {
        let z = i as u128;
        if n / z <= z * z {
            break;
        }
        let w = (n / z - z * z) / 3;
        if z * (z * z + 3 * w) == n {
            let y = ((z * z + 4 * w).sqrt() - z) / 2;
            let x = ((z * z + 4 * w).sqrt() + z) / 2;
            if x * x * x == n + y * y * y && y > 0 {
                println!("{} {}", x, y);
                return;
            }
        }
    }

    println!("-1");
}
