use proconio::input;
use proconio::marker::{Bytes, Usize1};

use std::collections::HashSet;
use std::isize::MIN;
use std::usize::MAX;

use std::collections::VecDeque;

use std::cmp::{max, min};

use itertools::*;


fn _01_bfs() {
    input! {
        n: usize, m: usize, k: usize,
        uv: [(Usize1, Usize1); m],
        a: [Usize1; n],
        b: [Usize1; k],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];

    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }

    let init_b_idx = if a[0] == b[0] { 1 } else { 0 };
    let mut q = VecDeque::from(vec![(0, init_b_idx)]);
    let mut visited = vec![false; n];

    let mut ans = false;
    while let Some((cur_node, b_index)) = q.pop_front() {

      if visited[cur_node] {
        continue;
      }
      visited[cur_node] = true;
      if cur_node == n - 1 {
          ans = b_index == k;
          break;
      }

      for &next in &g[cur_node] {
        if b_index < k && a[next] == b[b_index] {
          q.push_back((next, b_index + 1));
        } else {
          q.push_front((next, b_index));
        };
      }
    }
    println!("{}", if ans {"Yes"} else {"No"});
    
}
