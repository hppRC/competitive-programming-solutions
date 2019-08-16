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