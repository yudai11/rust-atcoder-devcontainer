use itertools::Itertools;
use proconio::{input, marker::Usize1};
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
        t: usize
    }

    let mut ans = vec![];

    for _i in 0..t {
        input! {
            n: usize,
            a: [Usize1; 2 * n]
        }

        let mut loc_cp = vec![vec![]; n];
        for (i, &ai) in a.iter().enumerate() {
            loc_cp[ai].push(i);
        }

        let mut res = 0_usize;
        for i in 0..n {
            let (x, y) = (loc_cp[i][0], loc_cp[i][1]);
            if y - x <= 1 {
                continue;
            }

            let mut gap = 0_usize;
            // 右の数字
            let b = a[x + 1];
            if b > i {
                let (z, w) = (loc_cp[b][1], loc_cp[b][0]);
                if z - w > 1
                    && ((z > y && z - y == 1) || (z < y && y - z == 1))
                    && ((w > x && w - x == 1) || (w < x && x - w == 1))
                {
                    gap += 1;
                    res += 1;
                }
            }

            if x > 0 {
                // 左の数字
                let b = a[x - 1];
                if b > i {
                    let (z, w) = (loc_cp[b][1], loc_cp[b][0]);
                    if z - w > 1
                        && ((z > y && z - y == 1) || (z < y && y - z == 1))
                        && ((w > x && w - x == 1) || (w < x && x - w == 1))
                    {
                        gap += 1;
                        res += 1;
                    }
                }
            }

            if gap == 2 && a[x - 1] == a[x + 1] {
                res -= 1;
            }
        }

        ans.push(res);
    }

    println!("{}", ans.iter().join("\n"));
}
