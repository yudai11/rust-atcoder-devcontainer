use proconio::input;

fn main() {
    input! {
        a: usize, b: usize
    }

    let c = gcd(a, b);
    // オーバーフローを回避するために条件の言い替えをする．
    if a / c > 1000_000_000_000_000_000 / b {
        println!("Large");
    } else {
        let ans = a / c * b;
        println!("{}", ans);
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let mut a = vec![x, y];
    loop {
        a.sort();
        if a[0] <= 1 {
            return 1;
        }
        let m = a[1] % a[0];
        if m == 0 {
            return a[0];
        }
        a[1] = m;
    }
}
