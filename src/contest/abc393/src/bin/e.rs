use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut m = a.iter().fold(0_usize, |res, &x| res.max(x));

    let mut occurences = vec![0_usize; m + 1];
    for &ai in a.iter() {
        occurences[ai] += 1;
    }

    let mut t = vec![0_usize; m + 1];
    for i in 1..=m {
        for j in (i..=m).step_by(i) {
            t[i] += occurences[j];
        }
    }

    let mut u = vec![0_usize; m + 1];
    for i in 1..=m {
        if t[i] < k {
            continue;
        }
        for j in (i..=m).step_by(i) {
            u[j] = u[j].max(i);
        }
    }

    for i in 0..n {
        println!("{}", u[a[i]]);
    }
}
