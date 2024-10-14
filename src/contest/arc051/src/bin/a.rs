use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// heap型の集合: .firstでmin,.lastでMAXを得られる。
// use std::collections::BTreeSet;

fn main() {
    input! {
        x1: isize, y1: isize, r: isize,
        x2: isize, y2: isize, x3: isize, y3: isize
    }

    let mut exist_red: bool;
    let mut exist_blue = false;
    // let long_edge = (x3 - x2).max(y3 - y2);

    if x2 <= x1 - r && x1 + r <= x3 && y2 <= y1 - r && y1 + r <= y3 {
        exist_red = false;
    } else {
        exist_red = true;
    }

    for x in x2..=x3 {
        for y in y2..=y3 {
            if (x - x1) * (x - x1) + (y - y1) * (y - y1) > r * r {
                exist_blue = true;
            }
        }
    }

    if exist_red {
        println!("YES");
    } else {
        println!("NO");
    }

    if exist_blue {
        println!("YES");
    } else {
        println!("NO");
    }
}
