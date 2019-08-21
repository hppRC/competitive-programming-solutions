use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Eratosthenes {
    end: usize,
    primes: Vec<usize>,
    flags: Vec<usize>,
}
#[allow(dead_code)]
impl Eratosthenes {
    fn new(n: usize) -> Self {
        let bits = 32;
        let flags_num = n / bits + 1;
        let defaults: Vec<usize> = vec![0x5D75D75D, 0xD75D75D7, 0x75D75D75];
        let (mut i, mut f, mut j);
        let max = ((n as f32).sqrt() as usize) + 1;
        let mut flags: Vec<usize> = (0..flags_num).map(|i| defaults[i % 3]).collect();
        flags[flags_num - 1] &= (1 << (n % bits + 1)) - 1 ;
        i = 5;
        f = 4;
        while i <= max {
            let t = (flags[i / bits] >> (i % bits)) & 1 == 1;
            if !t {
                j = i * (i | 1);
                while j <= n {
                    flags[j / bits] |= 1 << (j % bits);
                    j += i * 2;
                }
            }
            f = 6 - f;
            i += f;
        }
        flags[0] &= !0b1100;
        flags[0] |= 0b11;
        let mut primes = vec![];
        for p in 2..n+1 {
            if (flags[p / bits] >> (p % bits)) & 1 == 0 {
                primes.push(p);
            }
        }
        Eratosthenes {
            end: n,
            primes: primes,
            flags: flags,
        }
    }
    fn is_prime(&self, m: usize) -> bool {
        self.flags[m / 32] >> (m % 32) & 1 == 0
    }
    fn factorization(&self, n: usize) -> HashMap<usize, usize> {
        let mut n = n;
        let mut factors: HashMap<usize, usize> = HashMap::new();
        for &p in &self.primes {
            while n % p == 0 {
                n /= p;
                *factors.entry(p).or_insert(0) += 1;
            }
            if p > n {
                break;
            }
        }
        factors
    }
}

macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    let otogibara = Eratosthenes::new(10000000);
    debug!(otogibara);
    debug!(otogibara.is_prime(101));
    debug!(otogibara.factorization(5376));
}