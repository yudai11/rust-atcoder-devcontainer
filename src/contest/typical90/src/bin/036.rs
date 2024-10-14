use proconio::{input, marker::Usize1};
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut xy: Vec<(i64, i64)> = vec![(0, 0); n];

    {
        input! {
            _xy: [(i64,i64);n],
        }
        for i in 0..n {
            let (x, y) = _xy[i];
            xy[i].0 = x - y;
            xy[i].1 = x + y;
        }
    }

    let mut boundary_points: Vec<i64> = vec![0; 4];
    boundary_points[0] = xy[0].0;
    boundary_points[1] = xy[0].1;
    boundary_points[2] = xy[0].0;
    boundary_points[3] = xy[0].1;

    for i in 1..n {
        boundary_points[0] = boundary_points[0].max(xy[i].0);
        boundary_points[1] = boundary_points[1].max(xy[i].1);
        boundary_points[2] = boundary_points[2].min(xy[i].0);
        boundary_points[3] = boundary_points[3].min(xy[i].1);
    }

    input! {
        queries: [Usize1;q]
    }

    for i in 0..q {
        let qi = queries[i];
        let (x, y) = xy[qi];
        let ans = calc_dist(x, y, &boundary_points);
        println!("{ans}");
    }
}

fn calc_dist(x: i64, y: i64, boundary_points: &Vec<i64>) -> i64 {
    let mut ans: i64 = boundary_points[0] - x;
    ans = ans.max(boundary_points[1] - y);
    ans = ans.max(x - boundary_points[2]);
    ans = ans.max(y - boundary_points[3]);
    return ans;
}
