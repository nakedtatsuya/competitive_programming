use proconio::input;
use std::cmp::{min, max};
use std::usize::MAX;

fn main() {
    input! {
        n:usize,k:usize,
        a: [usize; k]
    }

    let mut dp = vec![vec![0; n+1]; 2];

    for j in 1..=n {
        { //* 先攻 */
            let mut now = 0;
            for x in a.iter() {
                if *x <= j {
                    now = max(now, dp[1][j-x] + x);
                }
            }
            dp[0][j] = now;
            println("{}", dp[1]);
        }

        { //* 後攻 */
            let mut now = MAX;
            for x in a.iter() {
                if *x <= j {
                    now = min(now, dp[0][j-x]);
                }
            }
            dp[1][j] = now;
        }
    }

    println!("{}", dp[0][n]);
    
}