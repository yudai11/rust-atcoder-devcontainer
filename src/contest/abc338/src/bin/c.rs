use proconio::input;

fn main() {
    input! {
        n: usize,
        mut q: [usize; n],
        a: [usize; n],
        b: [usize; n]
    }

    let mut ans = 0_usize;
    for i in 0..1000_001 {
        let mut num_b = 1000_000_usize;
        for j in 0..n {
            if b[j] > 0 {
                num_b = num_b.min(q[j] / b[j])
            }
        }
        ans = ans.max(i + num_b);

        let mut feasi = true;
        for j in 0..n {
            if q[j] < a[j] {
                feasi = false;
                break;
            }
        }

        if feasi {
            for j in 0..n {
                q[j] -= a[j];
            }
        } else {
            break;
        }
    }

    println!("{}", ans);
}
