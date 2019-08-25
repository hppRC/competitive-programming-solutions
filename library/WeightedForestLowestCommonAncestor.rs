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

fn main() {
    
}