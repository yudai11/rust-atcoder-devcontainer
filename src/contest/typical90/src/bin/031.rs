use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n]
    }

    let mut grundy_numbers = vec![vec![0_usize; 2501]; 51];

    for i in 0..=50 {
        for j in 0..=2500 {
            let mut seen = vec![0_usize; 2501];
            if i >= 1 && j + i <= 2500 {
                seen[grundy_numbers[i - 1][j + i]] = 1;
            }

            if j >= 2 {
                for k in 1..=j / 2 {
                    seen[grundy_numbers[i][j - k]] = 1;
                }
            }

            for k in 0..=j / 2 + 1 {
                if seen[k] == 0 {
                    grundy_numbers[i][j] = k;
                    break;
                }
            }
        }
    }

    let mut xor_sum = 0_usize;
    for i in 0..n {
        xor_sum ^= grundy_numbers[w[i]][b[i]];
    }

    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
