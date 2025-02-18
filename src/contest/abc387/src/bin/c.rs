use proconio::input;
// use proconio::marker::Chars;
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

fn main() {
    input! {
        l: usize,
        r: usize
    }

    println!("{}", f(r) - f(l - 1));
}

// x以下のヘビ数の数を返す関数
fn f(x: usize) -> usize {
    // 数字を文字列の配列にする
    let y = x.to_string().chars().collect::<Vec<_>>();
    // 文字列の配列を数字の配列にする
    let mut z = vec![];
    for &yi in y.iter() {
        z.push(yi as usize - '0' as usize);
    }
    let n = z.len();
    let mut res = 0_usize;
    // (i) xがヘビ数ならば+1
    let mut feasi = true;
    for i in 1..n {
        if z[i] >= z[0] {
            feasi = false;
            break;
        }
    }
    if feasi {
        res += 1;
    }
    // (ii) n桁の数であり上からk桁(0<= k < n-1)はxと一致しk+1桁目がxより小さいヘビ数
    for k in 0..n - 1 {
        // k桁目までがヘビ数の条件を満たすかチェック
        if k > 0 && z[k] >= z[0] {
            break;
        }
        if z[k + 1] == 0 {
            continue;
        }
        // k+1桁目は min(xのk+1桁目の数, xの1桁目の数) 通りの候補がある．
        let mut plus = z[k + 1].min(z[0]);
        for _i in k + 2..n {
            plus *= z[0];
        }
        res += plus;
    }
    // (iii) n桁の数であり上から1桁目がxより小さいヘビ数
    // 1桁目がi
    for i in 1..z[0] {
        let mut plus = 1_usize;
        for _j in 1..n {
            plus *= i;
        }
        res += plus;
    }
    // (iv) k桁のヘビ数(k < n)
    for k in 1..n {
        // 1桁目がi
        for i in 1..10 {
            let mut plus = 1_usize;
            for _j in 1..k {
                plus *= i;
            }
            res += plus;
        }
    }

    return res;
}
