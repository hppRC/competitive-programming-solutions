
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

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

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}


fn get_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}
fn readln<T>() -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug {
    get_line().parse().unwrap()
}


fn main() {
    let mut ans = vec![];
    let era = Eratosthenes::new(1299710);

    loop {
        let K: usize = readln();
        if K == 0 {
            break;
        }

        let i = era.primes.lower_bound(&K);

        if era.is_prime(K) {
            ans.push(0);
        }else {
            ans.push(era.primes[i] - era.primes[i-1]);
        }
    }

    for a in ans {
        println!("{}", a);
    }
}