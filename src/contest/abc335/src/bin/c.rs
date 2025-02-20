use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// priority que, peek,popでmax valを取り出せる(push(Reverse(x))とSome(Reverse(min_value)) = que.pop()でmin valを取れる)
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
        n: usize, q: usize
    }

    let mut dragon_sections = vec![];
    for i in 0..n {
        dragon_sections.push((i as isize + 1, 0_isize));
    }
    dragon_sections.reverse();

    // 頭の軌道を後ろに追加していく．
    let mut head_index = n - 1;
    for _i in 0..q {
        input! {
            t: u8
        }

        match t {
            1 => {
                input! {
                    c: char
                }
                let (x, y) = dragon_sections[head_index];
                match c {
                    'R' => {
                        dragon_sections.push((x + 1, y));
                    }
                    'L' => {
                        dragon_sections.push((x - 1, y));
                    }
                    'U' => {
                        dragon_sections.push((x, y + 1));
                    }
                    'D' => {
                        dragon_sections.push((x, y - 1));
                    }
                    _ => {}
                }
                head_index += 1;
            }
            2 => {
                input! {
                    p: usize
                }
                let (x, y) = dragon_sections[head_index + 1 - p];
                println!("{} {}", x, y);
            }
            _ => {}
        }
    }
}
