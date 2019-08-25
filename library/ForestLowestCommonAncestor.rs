#[derive(Eq, PartialEq, Clone, Debug)]
struct ForestLowestCommonAncestor {
    g: Vec<Vec<usize>>,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}
#[allow(dead_code)]
impl ForestLowestCommonAncestor {
    fn new(n: usize, edges: &[(usize, usize)]) -> Self
    {
        let mut g = vec![vec![]; n];
        for &(a, b) in edges.into_iter() {
            g[a].push(b);
            g[b].push(a);
        }
        let log_n = (1..).find(|i| (1usize << i) > n).unwrap();;
        let mut parent = vec![vec![None; n]; log_n];
        let mut depth = vec![0; n];
        let mut visit = vec![false; n];
        ForestLowestCommonAncestor::build(
            n,
            log_n,
            &g,
            &mut parent,
            &mut depth,
            &mut visit,
        );
        ForestLowestCommonAncestor {
            g: g,
            parent: parent,
            depth: depth,
        }
    }
    fn build(
        n: usize,
        log_n: usize,
        g: &[Vec<usize>],
        parent: &mut [Vec<Option<usize>>],
        depth: &mut [usize],
        visit: &mut [bool],
    ) -> () {
        for v in 0..n {
            if !visit[v] {
                ForestLowestCommonAncestor::dfs(v, None, 0, g, parent, depth, visit);
            }
        }
        for k in 0..log_n-1 {
            for v in 0..n {
                if parent[k][v] == None {
                    parent[k+1][v] = None;
                } else {
                    let super_parent = parent[k][v].unwrap();
                    parent[k+1][v] = parent[k][super_parent];
                }
            }
        }
    }
    fn dfs(
        v: usize,
        p: Option<usize>,
        d: usize,
        g: &[Vec<usize>],
        parent: &mut [Vec<Option<usize>>],
        depth: &mut [usize],
        visit: &mut [bool],
    ) -> () {
        parent[0][v] = p;
        depth[v] = d;
        visit[v] = true;
        for &u in &g[v] {
            if !visit[u] {
                ForestLowestCommonAncestor::dfs(u, Some(v), d+1, g, parent, depth, visit);
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
        if let Some(lca) = self.lca(a, b) {
            Some(self.depth[a] + self.depth[b] - 2 * self.depth[lca])
        } else {
            None
        }
    }
}

fn main() {

}