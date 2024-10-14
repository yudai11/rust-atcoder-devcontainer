use std::f64::consts::PI;

use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y1: f64,
        q: usize
    }

    for _ in 0..q {
        input! {
            mut e: f64
        }
        if e == t {
            e = 0.0;
        }
        let (y2, z) = (
            -l * ((e / t - 0.25) * 2.0 * PI).cos() / 2.0,
            l * (((e / t - 0.25) * 2.0 * PI).sin() + 1.0) / 2.0,
        );

        let xy_dist = (x * x + (y1 - y2) * (y1 - y2)).sqrt();
        let z_dist = z.abs();

        println!("{}", (z_dist / xy_dist).atan() * 180.0 / PI);
    }
}
