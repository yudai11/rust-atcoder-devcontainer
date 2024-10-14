use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashSet, VecDeque};
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use petgraph::unionfind::UnionFind;
// use std::collections::BinaryHeap;
// use proconio::marker::Isize1;
// use proconio::marker::Usize1;
// use std::cmp::Reverse;
// use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }

    let num_b_in_s = s
        .iter()
        .fold(0, |sum, &x| sum + if x == 'B' { 1 } else { 0 });
    let num_b_in_t = t
        .iter()
        .fold(0, |sum, &x| sum + if x == 'B' { 1 } else { 0 });

    // 碁石の数が揃わない
    if num_b_in_s != num_b_in_t {
        println!("-1");
        return;
    }

    let mut u = s.clone();
    u.push('.');
    u.push('.');
    // bfsを行うためのqueue
    let mut queue = VecDeque::new();
    // Save initial state of u as cost 0
    queue.push_back((u.iter().collect::<String>(), 0));
    // seenはすでに見た文字を保存
    let mut seen = HashSet::new();
    seen.insert(u.iter().collect::<String>());

    while queue.len() > 0 {
        let (v, cnt) = queue.pop_front().unwrap();
        let v_list = v.chars().collect::<Vec<char>>();
        let mut ok = true;
        for i in 0..n {
            if v_list[i] != t[i] {
                ok = false;
                break;
            }
        }
        // 一致した場合
        if ok {
            println!("{cnt}");
            return;
        }

        let mut pos_of_dot = 0;
        for i in 0..=n {
            if v_list[i] == '.' {
                pos_of_dot = i;
                break;
            }
        }
        for i in 0..=n {
            // ここの場合分けが難所
            if i == pos_of_dot || i + 1 == pos_of_dot || i == pos_of_dot + 1 {
                continue;
            }
            let mut next_v = v_list.clone();
            // let move_char = &v_list[i..(i + 1)];
            // next_v[pos_of_dot] = move_char[0];
            // next_v[pos_of_dot + 1] = move_char[1];
            // next_v[i] = '.';
            // next_v[i + 1] = '.';
            next_v.swap(i, pos_of_dot);
            next_v.swap(i + 1, pos_of_dot + 1);
            let next = next_v.iter().collect::<String>();
            if !seen.contains(&next) {
                queue.push_back((next.clone(), cnt + 1));
                seen.insert(next);
            }
        }
    }

    // 一致しなかった場合
    println!("-1");
}
