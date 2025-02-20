use proconio::input;
use std::collections::BTreeSet;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize, m: usize, mut loc: (isize,isize),
        xy: [(isize,isize); n],
        dc: [(char, isize); m]
    }

    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();
    for &(x, y) in xy.iter() {
        // x/yがkeyになければ平衡二分木を追加 -> その木にy/xを挿入
        x_map.entry(x).or_insert(BTreeSet::new()).insert(y);
        y_map.entry(y).or_insert(BTreeSet::new()).insert(x);
    }

    let mut ans = 0_usize;

    for &(d, c) in dc.iter() {
        match d {
            'U' => {
                let x = loc.0;
                let y = loc.1;
                loc.1 = y + c;

                if let Some(list) = x_map.get_mut(&x) {
                    let target_list = list.range(y..=y + c).cloned().collect::<Vec<isize>>(); // 範囲を指定して抜き出し
                    for yi in &target_list {
                        ans += 1;
                        if let Some(v) = y_map.get_mut(yi) {
                            v.remove(&x);
                        }
                        list.remove(yi);
                    }
                } else {
                    continue;
                }
            }
            'D' => {
                let x = loc.0;
                let y = loc.1;
                loc.1 = y - c;

                if let Some(list) = x_map.get_mut(&x) {
                    let target_list = list.range(y - c..=y).cloned().collect::<Vec<isize>>(); // 範囲を指定して抜き出し
                    for yi in &target_list {
                        ans += 1;
                        if let Some(v) = y_map.get_mut(yi) {
                            v.remove(&x);
                        }
                        list.remove(yi);
                    }
                } else {
                    continue;
                }
            }
            'R' => {
                let x = loc.0;
                let y = loc.1;
                loc.0 = x + c;

                if let Some(list) = y_map.get_mut(&y) {
                    let target_list = list.range(x..=x + c).cloned().collect::<Vec<isize>>(); // 範囲を指定して抜き出し
                    for xi in &target_list {
                        ans += 1;
                        if let Some(v) = x_map.get_mut(xi) {
                            v.remove(&y);
                        }
                        list.remove(xi);
                    }
                } else {
                    continue;
                }
            }
            'L' => {
                let x = loc.0;
                let y = loc.1;
                loc.0 = x - c;

                if let Some(list) = y_map.get_mut(&y) {
                    let target_list = list.range(x - c..x).cloned().collect::<Vec<isize>>(); // 範囲を指定して抜き出し
                    for xi in &target_list {
                        ans += 1;
                        if let Some(v) = x_map.get_mut(xi) {
                            v.remove(&y);
                        }
                        list.remove(xi);
                    }
                } else {
                    continue;
                }
            }
            _ => {}
        }
    }

    println!("{} {} {}", loc.0, loc.1, ans);
}
