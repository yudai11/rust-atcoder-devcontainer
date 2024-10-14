use ascii::ToAsciiChar;
use proconio::{input, marker::Chars};
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
        _n: usize, _m: usize,
        name: Chars,
        kit: Chars
    }

    let mut cnt_char_in_name = [0; 26];
    for &v in name.iter() {
        let index = v as usize - 'A' as usize;
        cnt_char_in_name[index] += 1;
    }

    let mut char_name_list = vec![];
    let mut num_alph_in_name = vec![];
    for i in 0..26 {
        if cnt_char_in_name[i] == 0 {
            continue;
        }

        char_name_list.push(('A' as u32 + i as u32).to_ascii_char().unwrap());
        num_alph_in_name.push(cnt_char_in_name[i]);
    }

    let k = char_name_list.len();
    let mut num_parts_in_kid = vec![0; k];
    for &v in kit.iter() {
        for i in 0..k {
            if v == char_name_list[i] {
                num_parts_in_kid[i] += 1;
                break;
            }
        }
    }

    let min_parts = num_parts_in_kid
        .iter()
        .fold(num_parts_in_kid[0], |min, &v| min.min(v));
    if min_parts == 0 {
        println!("-1");
    } else {
        let mut ans = 1;
        for i in 0..k {
            let mut needed_kit = num_alph_in_name[i] / num_parts_in_kid[i];
            if num_alph_in_name[i] % num_parts_in_kid[i] != 0 {
                needed_kit += 1;
            }
            ans = ans.max(needed_kit);
        }

        println!("{ans}");
    }
}
