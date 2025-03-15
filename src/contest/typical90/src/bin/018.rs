use itertools::Itertools;
use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64;q]
    }

    let mut ans = vec![];

    for &ei in e.iter() {
        let ei = ei % t;
        let angle = ei * 2.0 / t * PI;
        let loc_e8 = (0.0, -l / 2.0 * angle.sin(), l / 2.0 - l / 2.0 * angle.cos());
        let hori_gap = (x * x + (loc_e8.1 - y) * (loc_e8.1 - y)).sqrt();
        let verti_gap = loc_e8.2;
        ans.push(verti_gap.atan2(hori_gap) / PI * 180.0);
    }

    println!("{}", ans.iter().join("\n"));
}
