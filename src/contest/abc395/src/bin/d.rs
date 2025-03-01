use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: usize
    }

    // 鳩iの位置を記録
    let mut loc_pigeon = vec![0_usize; n];
    // 巣iの位置を記録
    let mut loc_nest = vec![0_usize; n];
    // 位置iの巣を記録
    let mut loc_nest_inv = vec![0_usize; n];
    for i in 0..n {
        loc_pigeon[i] = i;
        loc_nest[i] = i;
        loc_nest_inv[i] = i;
    }

    for _i in 0..q {
        input! {
            t: u8
        }
        match t {
            1 => {
                input! {
                    a: Usize1, b: Usize1
                }
                loc_pigeon[a] = loc_nest[b];
            }
            2 => {
                input! {
                    a: Usize1, b: Usize1
                }
                loc_nest_inv[loc_nest[a]] = b;
                loc_nest_inv[loc_nest[b]] = a;
                let t = loc_nest[a];
                loc_nest[a] = loc_nest[b];
                loc_nest[b] = t;
            }
            3 => {
                input! {
                    a: Usize1
                }
                println!("{}", loc_nest_inv[loc_pigeon[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
