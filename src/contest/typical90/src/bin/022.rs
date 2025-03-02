use proconio::input;

fn main() {
    input! {
        mut a: [isize;3]
    }

    let x = gcd(a[0], a[1]);
    let y = gcd(x, a[2]);

    a.sort();

    let ans = a.iter().fold(0_isize, |sum, &x| sum + x / y - 1);
    println!("{}", ans);
}

fn gcd(x: isize, y: isize) -> isize {
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
