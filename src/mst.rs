use proconio::input;
use std::cmp::{min, max};
use std::usize::MAX; 
fn main() {
    input! {
        n:usize, 
    }


    let mut g: Vec<Vec<usize>> = vec![vec![MAX; n]; n];

    for i in 0..n {
        for j in 0..n {
            input! {
                c: isize
            }
            if c >= 0 {
                g[i][j] = c as usize;
            }
        }
    }

    let mut color = vec![Color::White; n];
    let mut d = vec![MAX; n];
    let mut u: usize = 0;
    d[0] = 0;
    loop {
        let mut mincost = MAX;
        for i in 0..n {
            if color[i] != Color::Black && d[i] != MAX {
                d[i] = min(d[i], g[u][i]);
                if d[i] < mincost {
                    mincost = d[i];
                    u = i;
                }
            }
        }

        if mincost == MAX {
            break;
        }

        color[u] = Color::Black;

        for i in 0..n {
            if color[i] != Color::Black && g[u][i] != MAX {
                if g[u][i] < d[i] {
                    d[i] = g[u][i];
                    color[i] = Color::Gray;
                }
            }
        }
    }

    println!("{:?}", d.iter().sum::<usize>());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}
