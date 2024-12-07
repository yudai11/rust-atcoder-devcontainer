use proconio::{input, marker::Chars};
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
        h: usize, w: usize, d: usize,
        s: [Chars; h]
    }

    let mut ans: usize = 2;

    for x in 0..h * w {
        let c1 = (x / w, x % w);
        if s[c1.0][c1.1] == '#' {
            continue;
        }
        for y in x + 1..h * w {
            let c2 = (y / w, y % w);
            if s[c2.0][c2.1] == '#' {
                continue;
            }

            let mut pre_ans: usize = 0;
            let mut cp_s = s.clone();

            for i in 0..h {
                for j in 0..w {
                    if (c1.0 as isize - i as isize).abs() + (c1.1 as isize - j as isize).abs()
                        <= d as isize
                        && cp_s[i][j] == '.'
                    {
                        pre_ans += 1;
                        cp_s[i][j] = '#';
                    }
                    if (c2.0 as isize - i as isize).abs() + (c2.1 as isize - j as isize).abs()
                        <= d as isize
                        && cp_s[i][j] == '.'
                    {
                        pre_ans += 1;
                        cp_s[i][j] = '#';
                    }
                }
            }

            ans = ans.max(pre_ans);
        }
    }

    println!("{ans}");
}
