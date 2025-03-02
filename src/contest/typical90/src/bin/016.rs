use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [usize; 3]
    }

    v.sort();
    v.reverse();

    let a = v[0];
    let b = v[1];
    let c = v[2];

    let mut ans = 10000_usize;

    for i in 0..10000 {
        if i * a > n {
            break;
        }
        for j in 0..10000 {
            if i * a + j * b > n {
                break;
            }
            if (n - i * a - j * b) % c == 0 {
                ans = ans.min(i + j + (n - i * a - j * b) / c);
            }
        }
    }

    println!("{}", ans);
}
