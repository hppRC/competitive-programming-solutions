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
const MOD: usize = 1000000007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);
impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Dijkstra {
    dist: Vec<isize>,
    neighbors: Vec<Vec<(usize, isize)>>,
    n: usize,
}
impl Dijkstra {
    fn new(n: usize, edges: &Vec<(usize, usize, isize)>, s: usize) -> Self {
        let INF: isize = 1000000007;
        let mut dist: Vec<isize> = vec![INF; n];
        let mut neighbors: Vec<Vec<(usize, isize)>> = vec![vec![]; n];
        let mut heap: BinaryHeap<Rev<(usize, isize)>> = BinaryHeap::new();

        for &(a, b, c) in edges.into_iter() {
            neighbors[a].push((b, c));
        }

        dist[s] = 0;
        heap.push(Rev((s, 0)));

        while !heap.is_empty() {
            let Rev((v, d)) = heap.pop().unwrap();
            if dist[v] < d {
                continue;
            }
            for &(u, cost) in &neighbors[v] {
                if dist[u] > dist[v] + cost {
                    dist[u] = dist[v] + cost;
                    heap.push(Rev((u, cost)));
                }
            }
        }
        Dijkstra {
            dist: dist,
            neighbors: neighbors,
            n: n,
        }
    }
}


fn main() {
    input!{
        N: usize, M: usize, T: isize,
        A: [isize; N],
        abc: [(usize1, usize1, isize); M],
    }
    let revabc: Vec<(usize, usize, isize)> = abc.into_iter().map(|(a, b, c)| (b, a, c)).collect();

    let dk = Dijkstra::new(N, &abc, 0);
    let revdk = Dijkstra::new(N, &revabc, 0);

    let ans = (0..N).map(|i| (T - dk.dist[i] - revdk.dist[i]) * A[i]).max().unwrap();

    println!("{}", ans);
}