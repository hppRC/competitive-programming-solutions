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
macro_rules! printvec {
    ( $item:expr ) => {
        for &i in &$item {
            print!("{} ", i);
        }
        println!("");
    }
}
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
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
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Total<T>(pub T);
impl<T: PartialEq> Eq for Total<T> {}
impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
#[allow(dead_code)]
const MOD: usize = 1000000007;
#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    (1..n+1).into_iter().fold(1, |acc, i| acc * i)
}
#[allow(dead_code)]
fn comb(n: usize, r: usize) -> usize {
    if n - r < r {
        comb(n, n - r)
    } else {
        (1..r+1).into_iter().fold(1, |acc, i| acc * (n - r + i) / i)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
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
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct WeightedForestLowestCommonAncestor {
    g: Vec<Vec<(usize, usize)>>,
    parent: Vec<Vec<Option<usize>>>,
    weight: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}
#[allow(dead_code)]
impl WeightedForestLowestCommonAncestor {
    fn new<T>(n: usize, edges: &[(usize, usize, usize)], comp: T) -> Self
        where T: Fn(usize, usize) -> usize
    {
        let mut g = vec![vec![]; n];
        for &(a, b, cost) in edges.into_iter() {
            g[a].push((b, cost));
            g[b].push((a, cost));
        }
        let log_n = (1..).find(|i| (1usize << i) > n).unwrap();;
        let mut parent = vec![vec![None; n]; log_n];
        let mut weight = vec![vec![None; n]; log_n];
        let mut depth = vec![0; n];
        let mut visit = vec![false; n];
        WeightedForestLowestCommonAncestor::build(
            n,
            log_n,
            &g,
            &mut parent,
            &mut weight,
            &mut depth,
            &mut visit,
            &comp
        );
        WeightedForestLowestCommonAncestor {
            g: g,
            parent: parent,
            weight: weight,
            depth: depth,
        }
    }
    fn build<T>(
        n: usize,
        log_n: usize,
        g: &[Vec<(usize, usize)>],
        parent: &mut [Vec<Option<usize>>],
        weight: &mut [Vec<Option<usize>>],
        depth: &mut [usize],
        visit: &mut [bool],
        comp: T
    ) -> ()
        where T: Fn(usize, usize) -> usize
    {
        for v in 0..n {
            if !visit[v] {
                WeightedForestLowestCommonAncestor::dfs(v, None, 0, g, parent, weight, depth, visit);
            }
        }
        for k in 0..log_n-1 {
            for v in 0..n {
                if parent[k][v] == None {
                    parent[k+1][v] = None;
                } else {
                    let super_parent = parent[k][v].unwrap();
                    parent[k+1][v] = parent[k][super_parent];

                    if parent[k+1][v] != None {
                        let weight_p = weight[k][v].unwrap();
                        let weight_sp = weight[k][super_parent].unwrap();
                        weight[k+1][v] = Some(comp(weight_p, weight_sp));
                    } else {
                        weight[k+1][v] = None;
                    }
                }
            }
        }
    }
    fn dfs(
        v: usize,
        p: Option<usize>,
        d: usize,
        g: &[Vec<(usize, usize)>],
        parent: &mut [Vec<Option<usize>>],
        weight: &mut [Vec<Option<usize>>],
        depth: &mut [usize],
        visit: &mut [bool],
    ) -> () {
        parent[0][v] = p;
        depth[v] = d;
        visit[v] = true;
        for &(u, cost) in &g[v] {
            if !visit[u] {
                weight[0][u] = Some(cost);
                WeightedForestLowestCommonAncestor::dfs(u, Some(v), d+1, g, parent, weight, depth, visit);
            }
        }
    }
    fn lca(&self, mut a: usize, mut b: usize) -> Option<usize> {
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        while self.depth[a] != self.depth[b] {
            b = self.parent[(self.depth[b] - self.depth[a]).trailing_zeros() as usize][b].unwrap();
        }
        if a == b {
            return Some(a);
        }

        for k in (0..self.parent.len()).rev() {
            if self.parent[k][a] != self.parent[k][b] {
                a = self.parent[k][a].unwrap();
                b = self.parent[k][b].unwrap();
            }
        }
        if self.parent[0][a] == None && self.parent[0][b] == None {
            None
        } else {
            self.parent[0][a]
        }
    }
    fn dist(&self, a: usize, b: usize) -> Option<usize> {
        self.query_interface(a, b, |x, y| { x + y }, 0)
    }
    fn max_path_weight(&self, a: usize, b: usize) -> Option<usize> {
        self.query_interface(a, b, |x, y| { std::cmp::max(x, y) }, 0)
    }
    fn min_path_weight(&self, a: usize, b: usize) -> Option<usize> {
        let inf = 1000000007;
        self.query_interface(a, b, |x, y| { std::cmp::min(x, y) }, inf)
    }
    fn query_interface<T>(&self, mut a: usize, mut b: usize, f: T, init_ret: usize) -> Option<usize>
        where T: Fn(usize, usize) -> usize
    {
        if a == b {
            return None
        }
        let mut ret = init_ret;
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        while self.depth[a] != self.depth[b] {
            let step = (self.depth[b] - self.depth[a]).trailing_zeros() as usize;
            ret = f(ret, self.weight[step][b].unwrap());
            b = self.parent[step][b].unwrap();
        }
        if a == b {
            return Some(ret);
        }
        for k in (0..self.parent.len()).rev() {
            if self.parent[k][a] != self.parent[k][b] {
                ret = f(ret, self.weight[k][a].unwrap());
                ret = f(ret, self.weight[k][b].unwrap());
                a = self.parent[k][a].unwrap();
                b = self.parent[k][b].unwrap();
            }
        }
        if self.parent[0][a] == None && self.parent[0][b] == None {
            None
        } else {
            ret = f(ret, self.weight[0][a].unwrap());
            ret = f(ret, self.weight[0][b].unwrap());
            Some(ret)
        }
    }
}
#[allow(dead_code)]
struct WeightedMaxForestLowestCommonAncestor;
#[allow(dead_code)]
impl WeightedMaxForestLowestCommonAncestor {
    fn new(n: usize, edges: &[(usize, usize, usize)]) -> WeightedForestLowestCommonAncestor {
        WeightedForestLowestCommonAncestor::new(n, &edges, |x, y| { std::cmp::max(x, y) })
    }
}
#[allow(dead_code)]
struct WeightedMinForestLowestCommonAncestor;
#[allow(dead_code)]
impl WeightedMinForestLowestCommonAncestor {
    fn new(n: usize, edges: &[(usize, usize, usize)]) -> WeightedForestLowestCommonAncestor {
        WeightedForestLowestCommonAncestor::new(n, &edges, |x, y| { std::cmp::min(x, y) })
    }
}
#[allow(dead_code)]
struct WeightedDistForestLowestCommonAncestor;
#[allow(dead_code)]
impl WeightedDistForestLowestCommonAncestor {
    fn new(n: usize, edges: &[(usize, usize, usize)]) -> WeightedForestLowestCommonAncestor {
        WeightedForestLowestCommonAncestor::new(n, &edges, |x, y| { x + y })
    }
}



fn main() {
    input!{
        N: usize, M: usize,
        ab: [(usize1, usize1); M],
        Q: usize,
        xy: [(usize1, usize1); Q],
    }
    let mut uf = UnionFind::new(N);
    let mut edges = vec![];

    for (cost, (a, b)) in ab.into_iter().enumerate() {
        if !uf.same(a, b) {
            uf.unite(a, b);
            edges.push((a, b, cost));
        }
    }

    let lca = WeightedMaxForestLowestCommonAncestor::new(N, &edges);

    for &(x, y) in &xy {
        println!("{}", if let Some(ans) = lca.max_path_weight(x, y) { ans as isize + 1 } else { -1 });
    }

}