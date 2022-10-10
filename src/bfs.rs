use proconio::input;

use std::collections::HashSet;
use std::usize::MAX;
use std::isize::MIN;

use std::collections::VecDeque;

use std::cmp::{min, max};

use itertools::*;

fn main() {
    input! {
      n: usize,
      m: usize,
    }

    let mut map = vec![vec![MAX; n]; n];

    map[0][0] = 0;


    mv(&mut map, m, (0, 0));
    

    for i in 0..map.len() {
      for j in 0..map.len() {
        print!("{} ", map[i][j]);
      }
      print!("\n",);
    }

}

fn mv(map: &mut Vec<Vec<usize>>, m: usize, cur: (usize, usize)) {

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(cur);
    let mut dis: Vec<(isize, isize)> = vec![];
    while let Some(cur) = q.pop_front() {
      'outer: for i in 0..map.len() {
        for j in 0..map.len() {
          // ルートMになる座標を探す
          if map[i][j] != MAX {
            continue;
          }

          let mut y = (cur.0, i);
          let mut x = (cur.1, j);

          if y.0 < y.1 {
            y = (y.1, y.0);
          }

          if x.0 < x.1 {
            x = (x.1, x.0);
          }

          if (y.0-y.1)*(y.0-y.1) + (x.0-x.1)*(x.0-x.1) == m && map[i][j] > map[cur.0][cur.1] + 1 {

            dis = vec![(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
            break 'outer;
            // q.push_back((i, j));
            // map[i][j] = map[cur.0][cur.1] + 1;
          }
        }
      }



      


    }

    
}


// fn sum(i: usize, m: usize, a: &Vec<usize>, dp: &mut Vec<Vec<usize>>, n: usize) -> usize {
 
//   let mut tmp_max = 0;

//   if dp[i][m] != 0 {
//     return dp[i][m];
//   }

//   if m % 2 == 0 {
//     tmp_max = max(tmp_max, dp[i][m]);
//   }

//   if i > n {
//     return tmp_max;
//   }

//   tmp_max = max(sum(i+1, m, a, dp, n), sum(i+1, m+a[i], a, dp, n));

//   return tmp_max

// }

