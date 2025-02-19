use proconio::input;
use std::collections::HashMap;

// メモ化再帰 (状態数が少ないときに使う)
fn main() {
    input! {
        n: usize
    }

    let mut cost_map = HashMap::new();
    println!("{}", cost(n, &mut cost_map));
}

fn cost(n: usize, cost_map: &mut HashMap<usize, usize>) -> usize {
    if n < 2 {
        return 0;
    }
    if cost_map.contains_key(&n) {
        return cost_map[&n];
    } else {
        let y = n / 2;
        let z = if n % 2 == 0 { y } else { y + 1 };
        let res = cost(y, cost_map) + cost(z, cost_map) + n;
        cost_map.insert(n, res);
        return res;
    }
}
