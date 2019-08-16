#[derive(Debug, Clone)]
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
    
}