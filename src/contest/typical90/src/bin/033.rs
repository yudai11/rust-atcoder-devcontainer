use proconio::input;

fn main() {
    input! {
        h: usize, w: usize
    }

    let mut field = vec![vec![false; w]; h];
    let dd: [(isize, isize); 3] = [(-1, 0), (-1, -1), (0, -1)];
    let mut ans = 0_usize;

    if h == 1 || w == 1 {
        ans = h * w;
    } else {
        for i in 0..h {
            for j in 0..w {
                field[i][j] = true;
                for k in 0..3 {
                    if let Some((x, y)) = move_grid(i, j, dd[k].0, dd[k].1, h, w) {
                        if field[x][y] {
                            field[i][j] = false;
                            break;
                        }
                    }
                }
                if field[i][j] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
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
