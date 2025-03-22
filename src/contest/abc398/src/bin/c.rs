use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n]
    }

    let mut a = b.clone();

    a.sort();
    a.reverse();

    let mut max_val = a[0];
    let mut i = 1_usize;
    let mut no_ans = false;

    while i < n {
        let mut feasi = true;
        while max_val == a[i] {
            feasi = false;
            i += 1;
            if i == n {
                no_ans = true;
                feasi = true;
                break;
            }
        }

        if feasi {
            break;
        } else {
            max_val = a[i];
            i += 1;
        }
    }

    if no_ans {
        println!("-1");
        return;
    }

    for i in 0..n {
        if b[i] == max_val {
            println!("{}", i + 1);
            return;
        }
    }
}
