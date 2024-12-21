use std::collections::HashSet;

use proconio::input;
use proconio::marker::{Chars, Usize1};
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
        h: usize, w: usize, x: Usize1, y: Usize1,
        s: [Chars;h],
        t: Chars
    }

    let mut cur_place = x * w + y;
    let mut seen = HashSet::new();
    let mut ans: usize = 0;

    for &ti in t.iter() {
        if ti == 'U' {
            if cur_place / w > 0 {
                if s[cur_place / w - 1][cur_place % w] == '.' {
                    cur_place -= w;
                } else if s[cur_place / w - 1][cur_place % w] == '@' {
                    cur_place -= w;
                    if !seen.contains(&cur_place) {
                        seen.insert(cur_place);
                        ans += 1;
                    }
                }
            }
        } else if ti == 'D' {
            if cur_place / w + 1 < h {
                if s[cur_place / w + 1][cur_place % w] == '.' {
                    cur_place += w;
                } else if s[cur_place / w + 1][cur_place % w] == '@' {
                    cur_place += w;
                    if !seen.contains(&cur_place) {
                        seen.insert(cur_place);
                        ans += 1;
                    }
                }
            }
        } else if ti == 'L' {
            if cur_place % w > 0 {
                if s[cur_place / w][cur_place % w - 1] == '.' {
                    cur_place -= 1;
                } else if s[cur_place / w][cur_place % w - 1] == '@' {
                    cur_place -= 1;
                    if !seen.contains(&cur_place) {
                        seen.insert(cur_place);
                        ans += 1;
                    }
                }
            }
        } else if ti == 'R' {
            if cur_place % w + 1 < w {
                if s[cur_place / w][cur_place % w + 1] == '.' {
                    cur_place += 1;
                } else if s[cur_place / w][cur_place % w + 1] == '@' {
                    cur_place += 1;
                    if !seen.contains(&cur_place) {
                        seen.insert(cur_place);
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{} {} {}", cur_place / w + 1, cur_place % w + 1, ans);
}
