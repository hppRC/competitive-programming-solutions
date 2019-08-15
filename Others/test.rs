#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, char) => {
        read_value!($next, String).chars().collect::<Vec<char>>()[0]
     };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
    ($next:expr, isize1) => {
        read_value!($next, isize) - 1
    };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}
#[allow(dead_code)]
const MOD: usize = 1_000_000_007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}

#[derive(Debug, Clone)]
struct WarshallFloyd {
    dist: Vec<Vec<isize>>,
    next: Vec<Vec<usize>>,
    n: usize,
}
impl WarshallFloyd {
    fn new(n: usize, edges: &Vec<(usize, usize, isize)>) -> Self {
        let mut dist: Vec<Vec<isize>> = vec![vec![MOD as isize; n]; n];
        let mut next: Vec<Vec<usize>> = vec![vec![0; n]; n];
        for &(a, b, c) in edges.into_iter() {
            dist[a][b] = c;
            dist[b][a] = c;
        }
        for i in 0..n {
            dist[i][i] = 0;
        }
        for i in 0..n {
            for j in 0..n {
                next[i][j] = j;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
        WarshallFloyd {
            dist: dist,
            next: next,
            n: n
        }
    }
    #[allow(dead_code)]
    fn get_path(&mut self, i: usize, j: usize) -> Vec<usize> {
        let mut path = vec![];
        let mut to = self.next[i][j];
        path.push(i);
        while to != j {
            path.push(to);
            to = self.next[to][j];
        }
        path.push(to);
        path
    }
    #[allow(dead_code)]
    fn has_negative_loop(&mut self) -> bool {
        for i in 0..self.n {
            if self.dist[i][i] < 0 {
                return true
            }
        }
        false
    }
}

fn main() {
    input! {
        N: usize, M: usize,
        abc: [(usize1, usize1, isize); M],
    }
    //warshall floydして任意2点間最短距離と最短経路を復元できる構造体
    let mut waf = WarshallFloyd::new(N, &abc);
    let mut edges: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            let path = waf.get_path(i, j);
            for e in path.windows(2) {
                if !edges.contains(&(e[1], e[0])) {
                    edges.insert((e[0], e[1]));
                }
            }
        }
    }
    println!("{}", M - edges.into_iter().fold(0, |acc, _| acc + 1));
}