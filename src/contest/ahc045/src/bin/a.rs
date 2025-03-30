use itertools::Itertools;
use proconio::input;
use proconio::source::line::LineSource;
use std::io::{self, BufReader};

fn main() {
    // インタラクティブな問題でのinput の設定
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize, m: usize, q: usize, l: usize, w: usize,
        g: [usize; m],
        areas: [(usize,usize,usize,usize); n]
    }
    let mut centers = vec![];
    for (i, &(l1, r1, l2, r2)) in areas.iter().enumerate() {
        let mut c1 = (l1 + r1) / 2;
        let mut c2 = (l2 + r2) / 2;
        if l1 == 0 && r1 > w / 2 {
            c1 = r1 - (w / 2);
        } else if r1 == 10000 && l1 < 10000 - w / 2 {
            c1 = l1 + (w / 2);
        }
        if l2 == 0 && r2 > w / 2 {
            c2 = r2 - (w / 2);
        } else if r1 == 10000 && l1 < 10000 - w / 2 {
            c2 = l2 + (w / 2);
        }
        centers.push((c1, c2, i));
    }

    centers.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.cmp(&y.1)));

    let mut groups = vec![vec![]; m];
    let mut j = 0_usize;
    for i in 0..m {
        // if g[i] == 1 {
        //     continue;
        // }
        let capa = g[i];
        while groups[i].len() < capa {
            let (_x, _y, ind) = centers[j];
            groups[i].push(ind + 1);
            j += 1;
        }
    }

    let mut edges = vec![vec![]; m];
    let mut used = 0_usize;
    for i in 0..m {
        let capa = g[i];
        let mut seen = 1_usize;
        while seen <= capa {
            if used >= q {
                break;
            }
            used += 1;
            let to_see = l.min(capa + 1 - seen);
            let mut quest = vec![];
            for _k in 0..to_see {
                quest.push((groups[i][seen - 1], groups[i][seen]));
                seen += 1;
            }
            println!(
                "? {} {}",
                to_see,
                quest.iter().map(|x| format!("{} {}", x.0, x.1)).join(" ")
            );

            input! {
                ab: [(usize,usize); to_see - 1]
            }
            for &(a, b) in ab.iter() {
                edges[i].push((a, b));
            }
        }
    }

    println!("!");
    for i in 0..m {
        println!("{}", groups[i].iter().join(" "));
        println!(
            "{}",
            edges[i]
                .iter()
                .map(|x| format!("{} {}", x.0, x.1))
                .join("\n")
        );
    }
}
