#![allow(non_snake_case)]
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
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    group: Vec<usize>,
}
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
}



fn main() {
    input!{
        N: usize,
        M: usize,
        aby: [(usize1, usize1, usize); M],
        Q: usize,
        vw: [(usize1, usize); Q]
    }

    let mut aby = aby;
    let mut query: Vec<(usize, (usize, usize))> = vw.into_iter().enumerate().collect();

    aby.sort_by_key(|x| x.2);
    query.sort_by_key(|x| -((x.1).1 as isize));

    let mut uft = UnionFind::new(N);
    let mut ans = vec![0; Q];

    for (i, (v, w)) in query {
        while aby.last().map(|t| t.2 > w).unwrap_or(false) {
            let (a, b, _) = aby.pop().unwrap();
            uft.unite(a, b);
        }
        let p = uft.find(v);
        ans[i] = uft.group[p];
    }

    for a in ans {
        println!("{}", a);
    }
}
