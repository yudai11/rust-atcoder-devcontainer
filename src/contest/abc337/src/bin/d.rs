use proconio::{input, marker::Chars};
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
        h: usize, w: usize, k: usize,
        s: [Chars; h]
    }

    let mut ans = k + 2;

    for i in 0..h {
        // 尺取
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut cost = 0_usize;
        if s[i][right] == '.' {
            cost += 1;
        }
        while right < w && left + k - 1 < w {
            if s[i][right] == 'x' {
                right += 1;
                left = right;
                cost = 0;
                if right < w && s[i][right] == '.' {
                    cost += 1;
                }
                continue;
            } else if right - left >= k - 1 {
                ans = ans.min(cost);
                if s[i][left] == '.' {
                    cost -= 1;
                }
                left += 1;
            } else {
                right += 1;
                if s[i][right] == '.' {
                    cost += 1;
                }
            }
        }
    }

    for i in 0..w {
        // 尺取
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut cost = 0_usize;
        if s[right][i] == '.' {
            cost += 1;
        }
        while right < h && left + k - 1 < h {
            if s[right][i] == 'x' {
                right += 1;
                left = right;
                cost = 0;
                if right < h && s[right][i] == '.' {
                    cost += 1;
                }
                continue;
            } else if right - left >= k - 1 {
                ans = ans.min(cost);
                if s[left][i] == '.' {
                    cost -= 1;
                }
                left += 1;
            } else {
                right += 1;
                if s[right][i] == '.' {
                    cost += 1;
                }
            }
        }
    }

    if ans > k + 1 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
