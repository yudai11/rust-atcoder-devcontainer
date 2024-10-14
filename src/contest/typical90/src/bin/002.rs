use proconio::input;
// use proconio::marker::Chars;
// use itertools::Itertools;

fn main() {
    input! {
        mut n: usize,
    }

    if n % 2 != 0 {
        return;
    }

    for i in (0..(1 << n) as u64).rev() {
        if i.count_ones() != (n / 2) as u32 {
            continue;
        }

        let mut can_be_correct = true;
        let mut cum_sum = 0;

        for j in (0..n).rev() {
            if (i & (1 << j)) == 0 {
                cum_sum -= 1;
            } else {
                cum_sum += 1;
            }

            if cum_sum < 0 {
                can_be_correct = false;
                break;
            }
        }

        // println!("{}: {}", i, can_be_correct);

        if can_be_correct {
            for j in (0..n).rev() {
                if (i & (1 << j)) == 0 {
                    print!(")");
                } else {
                    print!("(");
                }
            }

            println!("")
        }
    }
}
