#[derive(Eq, PartialEq, Clone, Debug)]
struct FactInv {
    fact: Vec<usize>,
    inv: Vec<usize>,
    factinv: Vec<usize>,
    m: usize,
}
#[allow(dead_code)]
impl FactInv {
    fn new(n: usize, m: usize) -> Self {
        let mut fact: Vec<usize> = vec![0; n + 1];
        fact[0] = 1;
        for i in 1..n+1 {
            fact[i] = i * &fact[i - 1] % m;
        }
        let mut inv = vec![0; n + 1];
        inv[0] = 0;
        inv[1] = 1;
        for i in 2..n+1 {
            inv[i] = inv[m % i] * (m - m / i) % m;
        }
        let mut factinv = vec![0; n + 1];
        factinv[0] = 1;
        for i in 1..n+1 {
            factinv[i] = factinv[i - 1] * inv[i] % m;
        }
        FactInv {
            fact: fact,
            inv: inv,
            factinv: factinv,
            m: m,
        }
    }
    fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            0
        } else {
            (self.fact[n] * self.factinv[r] % self.m) * self.factinv[n-r] % self.m
        }
    }
}

const MOD: usize = 1000000007;

macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main () {
    let fi = FactInv::new(10, MOD);

    debug!(fi);
}