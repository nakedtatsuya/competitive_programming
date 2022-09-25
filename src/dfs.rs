use std::collections::VecDeque;

fn dfs_sinple_path(used: &mut Vec<bool>, g: &Vec<Vec<usize>>, stack: &mut VecDeque<usize>,  cur: usize, to: usize) {
  stack.push_back(cur);

  used[cur] = true;

  for next in g[cur].iter() {
    if used[*next] {
      continue;
    } else {
      dfs(used, g, stack, *next, to);
    }

    if stack[stack.len() - 1] == to {
      break;
    } else {
      stack.pop_back();
    }
  }
}

pub fn dfs_for_tree(g: &Vec<Vec<usize>>, ans: &mut VecDeque<usize>,  cur: usize, parent: isize, to: usize,) -> bool {
    if cur == to {
        ans.push_front(cur);
        return true;
    }
    for next in g[cur].iter() {
        if *next  as isize == parent {
            continue;
        } 

        if dfs(g, ans, *next, cur as isize, to) {
            ans.push_front(cur);
            return true;
        }
    }
    return false;
}