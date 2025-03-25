use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        txyv: [(usize,Usize1,Usize1,isize); q]
    }

    let mut potential: Vec<Option<isize>> = vec![None; n];

    // 各点の相対ポテンシャルを保存 (先読みして左から順に埋める)
    let mut read_list = vec![];
    for i in 0..q {
        let (t, x, y, v) = txyv[i];
        if t == 1 {
            continue;
        }
        read_list.push((x, y, v));
    }

    read_list.sort_by_key(|x| x.0);

    for &(x, y, v) in read_list.iter() {
        if let Some(a) = potential[x] {
            if let Some(b) = potential[y] {
                if a + b != v {
                    // return;
                }
            } else {
                potential[y] = Some(v - a);
            }
        } else {
            if let Some(b) = potential[y] {
                potential[x] = Some(v - b);
            } else {
                potential[x] = Some(0_isize);
                potential[y] = Some(v);
            }
        }
    }

    // println!(
    //     "{}",
    //     potential
    //         .iter()
    //         .map(|x| match x {
    //             Some(b) => &b,
    //             None => &0,
    //         })
    //         .join(" ")
    // );

    // 点の連結性を管理
    let mut uf = Dsu::new(n);
    let mut ans = vec![];

    for i in 0..q {
        let (t, x, y, v) = txyv[i];
        match t {
            0 => {
                uf.merge(x, y);
            }
            1 => {
                if uf.same(x, y) {
                    if (y + x) % 2 != 0 {
                        let res = potential[y].unwrap() + potential[x].unwrap() - v;

                        ans.push(res.to_string());
                    } else {
                        let res = potential[y].unwrap() + v - potential[x].unwrap();
                        ans.push(res.to_string());
                    }
                } else {
                    ans.push(String::from("Ambiguous"));
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().join("\n"));
}
