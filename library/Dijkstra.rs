use std::collections::BinaryHeap;
use std::cmp::Ordering;

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
#[allow(dead_code)]
impl Dijkstra {
    fn new(n: usize, edges: &Vec<(usize, usize, isize)>, s: usize) -> Self {
        let inf: isize = 1000000007;
        let mut dist: Vec<isize> = vec![inf; n];
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

}