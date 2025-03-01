use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut ans = vec![vec!['.'; n]; n];
    for i in 0..n {
        let j = n - i;
        if i > j {
            break;
        }
        match i % 2 {
            0 => {
                for k in i..j {
                    for l in i..j {
                        ans[k][l] = '#';
                    }
                }
            }
            1 => {
                for k in i..j {
                    for l in i..j {
                        ans[k][l] = '.';
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().map(|ai| ai.iter().join("")).join("\n"));
}
