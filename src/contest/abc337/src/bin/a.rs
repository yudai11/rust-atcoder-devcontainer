use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize);n]
    }

    let mut score_x = 0_usize;
    let mut score_y = 0_usize;
    for &(x, y) in xy.iter() {
        score_x += x;
        score_y += y;
    }

    if score_x > score_y {
        println!("Takahashi");
    } else if score_x < score_y {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
