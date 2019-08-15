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
const MOD: u64 = 1000000007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}

#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    group: Vec<usize>,
}
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
            group: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let px = self.par[x];
            let root = self.find(px);
            self.par[x] = root;
            root
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return
        }
        if self.rank[x] < self.rank[y] {
            self.group[y] += self.group[x];
            self.par[x] = y;
        } else {
            self.group[x] += self.group[y];
            self.par[y] = x;
        }
        if self.rank[x] == self.rank[y] {
            self.rank[y] += 1;
        }
    }
    fn group_size(&mut self, x: usize) -> usize {
        let p = self.find(x);
        self.group[p]
    }
}


fn main() {
    input!{
        N: usize, K: usize, L: usize,
        pq: [(usize1, usize1); K],
        rs: [(usize1, usize1); L],
    }
    let mut road = UnionFind::new(N);
    let mut train = UnionFind::new(N);

    let mut hash = HashMap::new();

    for &(p, q) in &pq {
        road.unite(p, q);
    }
    for &(r, s) in &rs {
        train.unite(r, s);
    }

    for i in 0..N {
        let x = road.find(i);
        let y = train.find(i);

        *hash.entry((x, y)).or_insert(0) += 1;
    }

    println!("{}",
        (0..N)
            .map(|i| {
                let x = road.find(i);
                let y = train.find(i);
                hash.get(&(x, y)).unwrap().to_string()
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}