use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        l: u64,
        k: u64,
        mut a: [u64; n],
    }

    a.push(l);

    let mut max: i64 = (l / (k as u64) + 1) as i64;
    let mut min = 0;
    // let mut ans: u64 = 0;

    loop {
        let test = min + (max - min) / 2;
        if test == min {
            break;
        }
        if is_larger_than_ans(&a, test, n + 1, k) {
            min = test;
        } else {
            max = test;
        }
    }

    println!("{}", min.max(0));
}

fn is_larger_than_ans(a: &Vec<u64>, test: i64, n: usize, k: u64) -> bool {
    let mut num_cut = 0;
    let mut current_locate: u64 = 0;
    for i in 0..n {
        if a[i] - current_locate < test as u64 {
            continue;
        } else if a[n - 1] - a[i] < test as u64 {
            break;
        } else {
            num_cut += 1;
            current_locate = a[i];
        }
    }

    if num_cut >= k {
        return true;
    } else {
        return false;
    }

    // if l - current_locate >= test {
    //     num_cut += 1;
    // }
    // if num_cut >= k {
    //     return true;
    // } else {
    //     return false;
    // }
}
