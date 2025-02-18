use std::collections::HashSet;

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
        n: usize, x: usize, y: usize,
        pt: [(usize,usize); n-1],
        q: usize,
        starts: [usize; q]
    }

    // // psの最小公倍数を求める．
    // let mut p_set = HashSet::new();
    // for &(p, _t) in pt.iter() {
    //     p_set.insert(p);
    // }
    // if p_set.contains(&1) {
    //     p_set.remove(&1);
    // }
    // let mut lcm = 1_usize;
    // for &pi in p_set.iter() {
    //     let mult = lcm * pi;
    //     let gcd = gcd(lcm, pi);
    //     lcm = mult / gcd;
    // }
    let lcm = 840; // Worst caseを通すことを考えると2~8のlcmを決め打ちしてもよい．

    let mut ans_mod_list = vec![0_usize; lcm];
    for i in 0..lcm {
        let mut sum: usize = i;
        sum += x;
        for i in 0..n - 1 {
            let (pi, ti) = pt[i];
            // while sum % pi != 0 {
            //     sum += 1;
            // }
            let res = sum % pi;
            if res > 0 {
                sum += pi - res;
            }
            sum += ti;
        }
        sum += y;
        sum -= i;
        ans_mod_list[i] = sum;
    }

    for stat in starts {
        println!("{}", stat + ans_mod_list[stat % lcm]);
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let mut a: Vec<usize> = vec![x, y];
    loop {
        a.sort();
        if a[0] <= 1 {
            return 1;
        }
        let m = a[1] % a[0];
        if m == 0 {
            return a[0];
        }
        a[1] = m;
    }
}
