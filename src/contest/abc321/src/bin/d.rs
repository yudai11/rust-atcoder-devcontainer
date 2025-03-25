use proconio::input;
use superslice::Ext; // for use of lowerbound upperbound method of vetor

fn main() {
    input! {
        n: usize, m: usize, p: usize,
        a: [usize; n],
        mut b: [usize; m]
    }

    b.sort();

    let mut cumsum_b = vec![0_usize; m];
    cumsum_b[0] = b[0];
    for i in 1..m {
        cumsum_b[i] = cumsum_b[i - 1] + b[i];
    }

    let mut ans = 0_usize;
    for i in 0..n {
        if a[i] >= p {
            ans += p * m;
            continue;
        }
        let ind = b.upper_bound(&(p - a[i] - 1));
        if ind <= 0 {
            ans += p * m;
            continue;
        }
        ans += cumsum_b[ind - 1] + a[i] * ind;
        ans += p * (m - ind);
    }

    println!("{}", ans);
}
