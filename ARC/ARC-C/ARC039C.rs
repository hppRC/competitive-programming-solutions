#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::io::{self, Write};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min, Ordering};
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
     };
     ($next:expr, $t:ty) => {
         $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(dead_code)]
const MOD: u64 = 1000000007;
#[allow(dead_code)]
fn to_num(c: char) -> i64 {
    c as i64 - 48
}


fn main() {
    input!{
        K: usize,
        S: chars
    }
    //direは[y, x]形式, U, R, D, Lの順
    let dire = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut hash = HashMap::new();

    hash.insert((0, 0), dire);
    let mut x = 0;
    let mut y = 0;

    for c in S {
        let to = match c {
            'U' => 0,
            'R' => 1,
            'D' => 2,
            'L' => 3,
            _ => unreachable!(),
        };
        let prevy = y;
        let prevx = x;

        while let Some(v) = hash.get(&(y, x)) {
            y = v[to].0;
            x = v[to].1;
        }

        if let Some(v) = hash.get_mut(&(prevy, prevx)) {
            v[to] = (y, x);
        }
        //(x, y)の上下左右方向に一つずつ見ていって各方向に辺を張る
        //hashの中にすでにある点が存在していたら既到達点なので(x,y)からその点までの直線移動時と同じ方向に結びつく
        //存在していなければ未到達点なのでその点を結びつける
        let u = if let Some(v) = hash.get(&(y + 1, x)) { v[0] } else { (y + 1, x) };
        let r = if let Some(v) = hash.get(&(y, x + 1)) { v[1] } else { (y, x + 1) };
        let d = if let Some(v) = hash.get(&(y - 1, x)) { v[2] } else { (y - 1, x) };
        let l = if let Some(v) = hash.get(&(y, x - 1)) { v[3] } else { (y, x - 1) };

        hash.insert((y, x), [u, r, d, l]);
    }
    println!("{} {}", x, y);
}