use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
        k: usize,
        mut a: [usize;n]
    }

    // 二分探索
    let mut left = 1;
    let mut right = l / (k + 1) + 1;

    loop {
        let mid = left + (right - left) / 2;
        if left == mid {
            break;
        }

        if can(mid, &a, l, n, k) {
            left = mid
        } else {
            right = mid
        }
    }

    println!("{}", left);
}

fn can(mid: usize, a: &Vec<usize>, l: usize, n: usize, k: usize) -> bool {
    let mut i = 0_usize;
    let mut cur_loc = 0_usize;
    let mut num_piece = 0_usize;
    while i < n {
        if a[i] - cur_loc >= mid {
            num_piece += 1;
            cur_loc = a[i];
        }
        i += 1;
    }
    if l - cur_loc >= mid {
        num_piece += 1;
    }

    if num_piece > k {
        return true;
    } else {
        return false;
    }
}
