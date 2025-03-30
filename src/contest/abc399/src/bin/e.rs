use proconio::{input, marker::Chars};
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
        _n: usize,
        s: Chars,
        t: Chars
    }

    let mut alph_locs = vec![vec![]; 26];
    for (i, &si) in s.iter().enumerate() {
        let ind = si as usize - 'a' as usize;
        alph_locs[ind].push(i);
    }

    for i in 0..26 {
        if alph_locs[i].len() == 0 {
            continue;
        }
        let init = t[alph_locs[i][0]] as usize - 'a' as usize;
        for j in 1..alph_locs[i].len() {
            if t[alph_locs[i][j]] as usize - 'a' as usize != init {
                println!("-1");
                return;
            }
        }
    }

    let mut to = vec![0_usize; n];
    let mut loc = vec![0_usize; n];
    let mut kind_s = 0_usize;
    let mut overlap = 0_usize;
    for i in 0..26 {
        loc[i] = i;
        if alph_locs[i].len() == 0 {
            to[i] = i;
        }
        let to_i = t[alph_locs[i][0]] as usize - 'a' as usize;
        to[i] = to_i;
    }

    let mut ans = 0_usize;
    while ans < 50 {
        for i in 0..26 {
            if to[i] == i {
                continue;
            }
            let next = to[i];
            let mut can_move = true;
            for j in 0..n {
                if to[j] == j {
                    continue;
                }
            }
        }
    }

    let mut kind_t = 0_usize;
    let mut flag_t = vec![false; 26];
    for &ti in t.iter() {
        let ind = ti as usize - 'a' as usize;
        if !flag_t[ind] {
            flag_t[ind] = true;
            kind_t += 1;
        }
    }

    let mut ans = 0_usize;
    for i in 0..26 {
        if alph_locs[i].len() == 0 {
            continue;
        }
        if s[alph_locs[i][0]] != t[alph_locs[i][0]] {
            ans += 1;

            // 移動先
            let b = t[alph_locs[i][0]] as usize - 'a' as usize;
            if alph_locs[b].len() == 0 {
                continue;
            }
            if alph_locs[b][0] > alph_locs[i][0] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    // if kind_s == overlap {
    //     println!("0");
    // } else if kind_s == kind_t && (kind_s - overlap) % 2 == 1 {
    //     println!("{}", kind_s - overlap + 1);
    // } else {
    //     println!("{}", kind_s - overlap);
    // }
}
