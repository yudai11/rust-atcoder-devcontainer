use std::collections::VecDeque;
// use std::process::exit;
use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize,usize,i64);n]
    }

    let mut dp = vec![vec![-1; w + 1]];
}

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
            return 0; // minなら i64::MAXにする
        }
        if left <= node_left && node_right <= right {
            return self.data[node];
        }
        let mid = (node_left + node_right) / 2;
        let left_res = self.query_range_rec(left, right, 2 * node, node_left, mid);
        let right_res = self.query_range_rec(left, right, 2 * node + 1, mid + 1, node_right);
        left_res.min(right_res) // minならmax -> min
    }
}
