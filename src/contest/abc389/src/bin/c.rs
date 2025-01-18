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
        q: usize
    }

    let mut minus_len: usize = 0;
    // 先頭の位置
    let mut first_ind: usize = 0;
    // headの位置と長さを記録
    let mut snakes: Vec<(usize, usize)> = vec![];
    let mut cur_loc: usize = 0;

    for _i in 0..q {
        input! {
            t: u8
        }
        if t == 1 {
            input! {
                l: usize
            }
            snakes.push((cur_loc, l));
            cur_loc += l;
        } else if t == 2 {
            minus_len += snakes[first_ind].1;
            first_ind += 1;
        } else {
            input! {
                k: Usize1
            }
            println!("{}", snakes[k + first_ind].0 - minus_len);
        }
    }
}
