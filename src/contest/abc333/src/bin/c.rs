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
        n: usize
    }

    let k = 12;
    let mut keta = vec![0_usize; k];
    let mut cnt_ok = 0_usize;

    loop {
        keta[0] += 1;
        // 繰り上げ
        loop {
            let mut can_stop = true;
            for i in 0..k - 1 {
                if keta[i] > 3 {
                    keta[i + 1] += 1;
                    keta[i] = 0;
                    can_stop = false;
                }
            }
            if can_stop {
                break;
            }
        }
        if keta.iter().sum::<usize>() == 3 {
            cnt_ok += 1;
        }
        if cnt_ok == n {
            break;
        }
    }

    let mut ans = 0_usize;
    for i in 0..k {
        if keta[i] > 0 {
            ans += keta[i] * repunit(i + 1);
        }
    }
    println!("{}", ans);
}

fn repunit(x: usize) -> usize {
    let mut res = 0_usize;
    let mut p = 1_usize;
    for _i in 0..x {
        res += p;
        p *= 10;
    }
    return res;
}
