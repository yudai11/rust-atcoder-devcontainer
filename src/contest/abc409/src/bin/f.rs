use ac_library::Dsu;
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

fn main() {
    input! {
        mut n: usize, q: usize,
        points: [(isize,isize); n],
    }

    let mut pos = vec![];

    for &(x, y) in points.iter() {
        pos.push((x - y, x + y));
    }

    let mut m = n;
    let mut uf = Dsu::new(size);

    for _i in 0..q {
        input! {
            t: u8
        }
        match t {
            1 => {
                input! {
                    (x,y): (isize,isize)
                }
                pos.push((x - y, x + y));
                n += 1;
                m += 1;
            }
            2 => {
                let mut groups = uf.groups();
            }
            3 => {}
            _ => unreachable!(),
        }
    }

    let mut boundary_points: Vec<i64> = vec![0; 4];
    boundary_points[0] = xy[0].0;
    boundary_points[1] = xy[0].1;
    boundary_points[2] = xy[0].0;
    boundary_points[3] = xy[0].1;

    for i in 1..n {
        boundary_points[0] = boundary_points[0].max(xy[i].0);
        boundary_points[1] = boundary_points[1].max(xy[i].1);
        boundary_points[2] = boundary_points[2].min(xy[i].0);
        boundary_points[3] = boundary_points[3].min(xy[i].1);
    }

    input! {
        queries: [Usize1;q]
    }

    for i in 0..q {
        let qi = queries[i];
        let (x, y) = xy[qi];
        let ans = calc_dist(x, y, &boundary_points);
        println!("{ans}");
    }
}

fn calc_dist(x: i64, y: i64, boundary_points: &Vec<i64>) -> i64 {
    let mut ans: i64 = 0;
    ans = boundary_points[0] - x;
    ans = ans.max(boundary_points[1] - y);
    ans = ans.max(x - boundary_points[2]);
    ans = ans.max(y - boundary_points[3]);
    return ans;
}
