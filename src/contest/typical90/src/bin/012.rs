use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize, w: usize,
        q: usize
    }

    // Disjoint Set Union (Atcoder Library)
    let mut dsu = Dsu::new(h * w);
    // 赤で塗られている箇所を管理
    let mut field = vec![vec![false; w]; h];

    let dx: [isize; 4] = [0, 0, -1, 1];
    let dy: [isize; 4] = [-1, 1, 0, 0];

    for _i in 0..q {
        input! {
            t: usize
        }

        match t {
            1 => {
                input! {
                    (r,c): (Usize1,Usize1)
                }
                if field[r][c] {
                    continue;
                }
                field[r][c] = true;
                for j in 0..4 {
                    if let Some((x, y)) = move_grid(r, c, dx[j], dy[j], h, w) {
                        if field[x][y] {
                            dsu.merge(r * w + c, x * w + y);
                        }
                    }
                }
            }
            2 => {
                input! {
                    (r1,c1,r2,c2): (Usize1,Usize1,Usize1,Usize1)
                }
                if !field[r1][c1] || !field[r2][c2] || !dsu.same(r1 * w + c1, r2 * w + c2) {
                    println!("No");
                } else {
                    println!("Yes");
                }
            }
            _ => unreachable!(),
        }
    }
}

// gridの移動ができるならばその中身を返す関数
fn move_grid(
    i: usize,
    j: usize,
    dx: isize,
    dy: isize,
    h: usize,
    w: usize,
) -> Option<(usize, usize)> {
    if i as isize + dx >= 0
        && i as isize + dx < h as isize
        && j as isize + dy >= 0
        && j as isize + dy < w as isize
    {
        return Some(((i as isize + dx) as usize, (j as isize + dy) as usize));
    } else {
        return None;
    }
}
