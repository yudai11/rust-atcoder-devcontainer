use proconio::input;
use proconio::marker::Chars;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
// use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        h: usize, w: usize, d: usize,
        s: [Chars; h]
    }

    let mut when_humidified: Vec<Vec<usize>> = vec![vec![d + 2; w]; h];
    let mut to_see = BinaryHeap::new();
    let mut ans: usize = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'H' {
                when_humidified[i][j] = 0;
                to_see.push((Reverse(0_usize), i, j));
                ans += 1;
            }
        }
    }

    let dx: [isize; 4] = [0, 0, 1, -1];
    let dy: [isize; 4] = [1, -1, 0, 0];

    while !to_see.is_empty() {
        let p = to_see.pop().unwrap();
        if (p.0).0 >= d {
            continue;
        }
        for i in 0..4 {
            // 移動できるとき
            if p.1 as isize + dx[i] >= 0
                && p.1 as isize + dx[i] < h as isize
                && p.2 as isize + dy[i] >= 0
                && p.2 as isize + dy[i] < w as isize
                && s[(p.1 as isize + dx[i]) as usize][(p.2 as isize + dy[i]) as usize] != '#'
            {
                if when_humidified[(p.1 as isize + dx[i]) as usize][(p.2 as isize + dy[i]) as usize]
                    > d + 1
                {
                    ans += 1;
                }

                // 最短経路の更新ができるなら再び入れる <- 重要，入れないと参照回数が重複しすぎて終わる
                if (p.0).0
                    < when_humidified[(p.1 as isize + dx[i]) as usize]
                        [(p.2 as isize + dy[i]) as usize]
                {
                    when_humidified[(p.1 as isize + dx[i]) as usize]
                        [(p.2 as isize + dy[i]) as usize] = (p.0).0;

                    // まだ移動の余地がある．
                    if (p.0).0 + 1 < d {
                        to_see.push((
                            Reverse((p.0).0 + 1),
                            (p.1 as isize + dx[i]) as usize,
                            (p.2 as isize + dy[i]) as usize,
                        ))
                    }
                }
            }
        }
    }

    println!("{ans}");
}
