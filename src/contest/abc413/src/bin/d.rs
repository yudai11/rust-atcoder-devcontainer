use itertools::Itertools;
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

// 等比数列
fn main() {
    input! {
        t: usize,
    }

    let mut ans = vec![];

    for _i in 0..t {
        input! {
            n: usize,
            mut a: [isize; n]
        }

        a.sort_by(|&x, &y| (x.abs()).cmp(&(y.abs())));
        // println!("{}", a.iter().join(" "));

        let mut feasi = true;

        for i in 1..n - 1 {
            if a[i - 1] * a[i + 1] != a[i] * a[i] {
                feasi = false;
                break;
            }
        }

        if a[0] * a[1] < 0 {
            for i in 0..n - 1 {
                if a[i] * a[i + 1] > 0 {
                    feasi = false;
                    break;
                }
            }
        }

        if a[0] * a[1] > 0 {
            for i in 0..n - 1 {
                if a[i] * a[i + 1] < 0 {
                    feasi = false;
                    break;
                }
            }
        }

        if a[0].abs() == a[1].abs() {
            feasi = true;
            let mut plus = 0_isize;
            let mut minus = 0_isize;
            for i in 1..n {
                if a[i].abs() != a[0].abs() {
                    feasi = false;
                    break;
                } else {
                    if a[i] > 0 {
                        plus += 1;
                    } else {
                        minus += 1;
                    }
                }
            }

            if (plus - minus).abs() > 1 && plus != 0 && minus != 0 {
                feasi = false;
            }
        }

        if feasi {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", ans.iter().join("\n"));
}
