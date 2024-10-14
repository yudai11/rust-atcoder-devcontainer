use std::collections::{HashMap, HashSet};

use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

struct LazySegmentTree {
    n: usize,
    data: Vec<i64>,
    lazy: Vec<Option<i64>>, // Noneは遅延がないことを示す
}

impl LazySegmentTree {
    fn new(size: usize) -> Self {
        let n = size.next_power_of_two();
        LazySegmentTree {
            n,
            data: vec![0; 2 * n],
            lazy: vec![None; 2 * n],
        }
    }

    // 遅延配列を伝播して適用
    fn propagate(&mut self, node: usize, node_left: usize, node_right: usize) {
        if let Some(value) = self.lazy[node] {
            self.data[node] = value;
            if node_left != node_right {
                self.lazy[2 * node] = Some(value);
                self.lazy[2 * node + 1] = Some(value);
            }
            self.lazy[node] = None;
        }
    }

    // 範囲更新: 指定の範囲をvalueに設定
    fn update_range(&mut self, left: usize, right: usize, value: i64) {
        self.update_range_rec(left, right, value, 1, 0, self.n - 1);
    }

    fn update_range_rec(
        &mut self,
        left: usize,
        right: usize,
        value: i64,
        node: usize,
        node_left: usize,
        node_right: usize,
    ) {
        self.propagate(node, node_left, node_right);
        if right < node_left || node_right < left {
            return;
        }
        if left <= node_left && node_right <= right {
            self.lazy[node] = Some(value);
            self.propagate(node, node_left, node_right);
        } else {
            let mid = (node_left + node_right) / 2;
            self.update_range_rec(left, right, value, 2 * node, node_left, mid);
            self.update_range_rec(left, right, value, 2 * node + 1, mid + 1, node_right);
            self.data[node] = self.data[2 * node].max(self.data[2 * node + 1]);
        }
    }

    // 範囲クエリ: 指定の範囲の最大値を取得
    fn query_range(&mut self, left: usize, right: usize) -> i64 {
        self.query_range_rec(left, right, 1, 0, self.n - 1)
    }

    fn query_range_rec(
        &mut self,
        left: usize,
        right: usize,
        node: usize,
        node_left: usize,
        node_right: usize,
    ) -> i64 {
        self.propagate(node, node_left, node_right);
        if right < node_left || node_right < left {
            return 0;
        }
        if left <= node_left && node_right <= right {
            return self.data[node];
        }
        let mid = (node_left + node_right) / 2;
        let left_res = self.query_range_rec(left, right, 2 * node, node_left, mid);
        let right_res = self.query_range_rec(left, right, 2 * node + 1, mid + 1, node_right);
        left_res.max(right_res)
    }
}

fn press_coordinate(list: &Vec<usize>) -> Vec<usize> {
    let mut copy_set = HashSet::new();
    for &v in list {
        copy_set.insert(v);
    }

    let mut copy_list = vec![];
    for &x in copy_set.iter() {
        copy_list.push(x);
    }
    copy_list.sort();

    let mut press_map: HashMap<usize, usize> = HashMap::new();
    for (i, &k) in copy_list.iter().enumerate() {
        press_map.insert(k, i);
    }

    let mut pressed_list: Vec<usize> = vec![];
    for x in list {
        pressed_list.push(press_map[x]);
    }

    pressed_list
}

fn main() {
    input! {
        _w: usize,
        n: usize,
        _lr: [usize;2 * n]
    }

    // 座標を圧縮
    let new_list = press_coordinate(&_lr);
    let mut lr: Vec<(usize, usize)> = vec![(0, 0); n];
    for i in 0..n {
        lr[i].0 = new_list[2 * i];
        lr[i].1 = new_list[2 * i + 1];
    }
    // 座標を圧縮

    let w = new_list.iter().fold(n, |x, &y| y.max(x));
    let mut seg_tree = LazySegmentTree::new(w);
    for &(l, r) in &lr {
        let h = seg_tree.query_range(l, r);
        println!("{}", h + 1);
        seg_tree.update_range(l, r, h + 1);
    }
}
