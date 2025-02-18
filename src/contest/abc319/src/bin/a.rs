// use itertools::Itertools;
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
    input! {
        s: str
    }

    // let s = s.as_str();

    match s {
        "tourist" => {
            println!("3858");
        }
        "ksun48" => {
            println!("3679");
        }
        "Benq" => {
            println!("3658");
        }
        "Um_nik" => {
            println!("3648");
        }
        "apiad" => {
            println!("3638");
        }
        "Stonefeang" => {
            println!("3630");
        }
        "ecnerwala" => {
            println!("3613");
        }
        "mnbvmar" => {
            println!("3555");
        }
        "newbiedmy" => {
            println!("3516");
        }
        "semiexp" => {
            println!("3481");
        }
        _ => {}
    }
}
