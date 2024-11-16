use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::ascii::AsciiExt;
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
        s: Chars,
        q: usize,
        k: [Usize1; q]
    }

    let mut ans = vec!['a'; q];

    // 裏表を管理
    // let mut x = s.len();
    // let mut pow2_s = vec![x; 1];
    // while x < 1000000000000000006 {
    //     x *= 2;
    //     pow2_s.push(x);
    // }

    for &ki in k.iter() {
        let local = ki % s.len();
        let mut global = ki / s.len();
        let mut ura_omote = 0;
        while global > 0 {
            if global % 2 == 1 {
                ura_omote += 1;
                ura_omote %= 2;
            }

            global /= 2;
        }

        let mut ans: char = s[local];
        if ura_omote == 1 {
            if ans.is_uppercase() {
                ans = ans.to_ascii_lowercase();
            } else {
                ans = ans.to_ascii_uppercase();
            }
        }

        print!("{ans} ");
    }
}
