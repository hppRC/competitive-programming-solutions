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


fn main() {
    input! {
        N: u64,
        A: u64,
        B: u64
    }

    let ans = (1..N+1)
        .filter(|&i| {
            let mut x = i;
            let mut sum = 0;

            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            A <= sum && sum <= B
        })
        .sum::<u64>();
    println!("{}", ans);
}


/*
fn main() {
    input! {
        N: u64,
        A: u64,
        B: u64
    }
    let ans = (1..N+1)
        .filter(|i| {
            let sum = i.to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as u64)
                .sum::<u64>();
            A <= sum && sum <= B
        })
        .sum::<u64>();
    println!("{}", ans);
}
*/