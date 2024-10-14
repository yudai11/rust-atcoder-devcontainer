use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
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
        x: [isize;n],
        p: [usize;n],
        q: usize,
        lr: [(isize, isize);q]
    }

    let mut press_map: HashMap<isize, usize> = HashMap::new();
    let mut m = 0;

    {
        let mut list = vec![];
        for i in 0..n {
            list.push(x[i]);
        }
        for i in 0..q {
            let (l, r) = lr[i];
            list.push(l);
            list.push(r);
        }
        let mut copy_set = HashSet::new();
        for &v in &list {
            copy_set.insert(v);
        }
        let mut copy_list = vec![];
        for &x in copy_set.iter() {
            copy_list.push(x);
        }
        copy_list.sort();
        for (i, &k) in copy_list.iter().enumerate() {
            press_map.insert(k, i);
        }
        let mut pressed_list: Vec<usize> = vec![];
        for x in list {
            pressed_list.push(press_map[&x]);
        }
        m = pressed_list.len();
    }

    let mut vils = vec![0; m];
    for i in 0..n {
        vils[press_map[&x[i]]] = p[i];
    }

    let mut b: Vec<u128> = vec![0; m + 1];
    for i in 0..m {
        b[i + 1] = b[i] + vils[i] as u128;
    }
    for i in 0..q {
        let (l, r) = (press_map[&lr[i].0], press_map[&lr[i].1]);
        println!("{}", b[r + 1] - b[l]);
    }
}

fn press_map(list: &Vec<isize>) -> HashMap<isize, usize> {
    let mut copy_set = HashSet::new();
    for &v in list {
        copy_set.insert(v);
    }

    let mut copy_list = vec![];
    for &x in copy_set.iter() {
        copy_list.push(x);
    }
    copy_list.sort();

    let mut press_map: HashMap<isize, usize> = HashMap::new();
    for (i, &k) in copy_list.iter().enumerate() {
        press_map.insert(k, i);
    }
    press_map
}

fn press_coordinate(list: &Vec<isize>) -> Vec<usize> {
    let mut copy_set = HashSet::new();
    for &v in list {
        copy_set.insert(v);
    }

    let mut copy_list = vec![];
    for &x in copy_set.iter() {
        copy_list.push(x);
    }
    copy_list.sort();

    let mut press_map: HashMap<isize, usize> = HashMap::new();
    for (i, &k) in copy_list.iter().enumerate() {
        press_map.insert(k, i);
    }

    let mut pressed_list: Vec<usize> = vec![];
    for x in list {
        pressed_list.push(press_map[x]);
    }

    pressed_list
}
