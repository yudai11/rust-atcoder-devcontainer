use itertools::Itertools;
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
        c: [usize;9]
    }

    // がっかりする順列の数
    let mut sum = 0_usize;

    for p in (0..9 as usize).permutations(9) {
        let mut tate = vec![vec![]; 3];
        let mut yoko = vec![vec![]; 3];
        let mut naname = vec![vec![]; 2];
        // 順列piでがっかりするかどうか判定
        let mut disappointment = false;
        for pi in p {
            // tate
            if tate[pi % 3].len() == 0 {
                tate[pi % 3].push(c[pi]);
            } else if tate[pi % 3].len() == 1 {
                if tate[pi % 3][0] == c[pi] {
                    disappointment = true;
                    break;
                } else {
                    tate[pi % 3].push(c[pi]);
                }
            }

            // if disappointment {
            //     break;
            // }

            // yoko
            if yoko[pi / 3].len() == 0 {
                yoko[pi / 3].push(c[pi]);
            } else if yoko[pi / 3].len() == 1 {
                if yoko[pi / 3][0] == c[pi] {
                    disappointment = true;
                    break;
                } else {
                    yoko[pi / 3].push(c[pi]);
                }
            }

            // if disappointment {
            //     break;
            // }

            //naname
            if pi == 0 || pi == 4 || pi == 8 {
                if naname[0].len() == 0 {
                    naname[0].push(c[pi]);
                } else if naname[0].len() == 1 {
                    if naname[0][0] == c[pi] {
                        disappointment = true;
                        break;
                    } else {
                        naname[0].push(c[pi]);
                    }
                }
            }

            if pi == 2 || pi == 4 || pi == 6 {
                if naname[1].len() == 0 {
                    naname[1].push(c[pi]);
                } else if naname[1].len() == 1 {
                    if naname[1][0] == c[pi] {
                        disappointment = true;
                        break;
                    } else {
                        naname[1].push(c[pi]);
                    }
                }
            }
        }

        if disappointment {
            sum += 1;
        }
    }

    let all_perm: usize = 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9;
    let ans = (all_perm - sum) as f64 / all_perm as f64;

    println!("{}", ans);
}
