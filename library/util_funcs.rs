#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};

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
#[allow(dead_code)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
#[allow(dead_code)]
fn lcm(a: usize, b:usize) -> usize {
    a * b / gcd(a, b)
}
#[allow(dead_code)]
fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    let mut res = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n = n >> 1;
    }
    res
}
#[allow(dead_code)]
fn mod_inv(a: usize, m: usize) -> usize {
    let (_, x, _) = extgcd(a as i64, m as i64);
    (m as i64 + x) as usize % m
}
#[allow(dead_code)]
fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}


fn main() {
    println!("{}", factorial(5));
    println!("{}", comb(5,3));
    println!("{}", gcd(16, 6));
    println!("{}", lcm(5, 6));
    println!("{}", mod_pow(2, 10, 1000)); //1024 % 1000
    println!("{}", mod_inv(8, 13));
    println!("{:?}", extgcd(4, 7));
}