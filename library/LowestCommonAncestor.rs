#[derive(Eq, PartialEq, Clone, Debug)]
struct LowestCommonAncestor {
    g: Vec<Vec<usize>>,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}
impl LowestCommonAncestor {
    fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut g = vec![vec![]; n];
        for &(a, b) in edges.into_iter() {
            g[a].push(b);
            g[b].push(a);
        }
        let log_n = (1..).find(|i| (1usize << i) > n).unwrap();;
        let mut parent = vec![vec![None; n]; log_n];
        let mut depth = vec![0; n];
        LowestCommonAncestor::build(
            n,
            log_n,
            &g,
            &mut parent,
            &mut depth,
        );
        LowestCommonAncestor {
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
    ) -> () {
        LowestCommonAncestor::dfs(0, None, 0, g, parent, depth);

        for k in 0..log_n-1 {
            for v in 0..n {
                if parent[k][v] == None {
                    parent[k+1][v] = None;
                } else {
                    parent[k+1][v] = parent[k][parent[k][v].unwrap()];
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
    ) -> () {
        parent[0][v] = p;
        depth[v] = d;
        for &u in &g[v] {
            if Some(u) != p {
                LowestCommonAncestor::dfs(u, Some(v), d+1, g, parent, depth);
            }
        }
    }

    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] > self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        while self.depth[a] != self.depth[b] {
            b = self.parent[(self.depth[b] - self.depth[a]).trailing_zeros() as usize][b].unwrap();
        }
        if a == b {
            return a;
        }

        for k in (0..self.parent.len()).rev() {
            if self.parent[k][a] != self.parent[k][b] {
                a = self.parent[k][a].unwrap();
                b = self.parent[k][b].unwrap();
            }
        }
        self.parent[0][a].unwrap()
    }
}