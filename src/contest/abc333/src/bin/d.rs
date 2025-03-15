use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        edges: [(Usize1,Usize1); n-1]
    }
    let mut dsu = Dsu::new(n - 1);
    for &(u, v) in edges.iter() {
        if u == 0 || v == 0 {
            continue;
        }
        dsu.merge(u - 1, v - 1);
    }

    let x = dsu.groups();
    let mut a = vec![];
    for xi in x.iter() {
        a.push(xi.len());
    }

    a.sort();
    a.reverse();

    if a.len() > 1 {
        let sum = a.iter().sum::<usize>();
        println!("{}", sum - a[0] + 1);
    } else {
        println!("1");
    }
}
