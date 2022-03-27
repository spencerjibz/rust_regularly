use num::{bigint::BigUint, integer::Integer, iter, One};
use regex::Regex;

use std::time::Instant;
fn main() {
    let now = Instant::now();
    //println!("{:?}", convert_fracts(vec![(690, 1300), (87, 1310), (30, 40)]));
    println!("{:?}", decomp(12));
    println!("{:?}", now.elapsed())
}

#[allow(dead_code)]
fn zeros(n: u64) -> u64 {
    //get tring
    let a = fac(n).to_str_radix(10);
    let re = Regex::new(r"0+$").unwrap();
    let v = match re.captures(&a) {
        Some(ele) => ele.get(0).map_or("", |m| m.as_str()),
        None => "",
    };
    //println!("{:?},{}",v,a);
    v.chars().count() as u64
}

fn fac(value: u64) -> BigUint {
    (1..=value).fold(BigUint::one(), |res, n| res * n)
}
#[allow(dead_code)]
fn n_zeros(mut n: u64) -> u64 {
    let mut sz = 0;
    while n > 0 {
        n = (n).div_floor(&5);
        sz += n;
    }
    sz
}
#[allow(dead_code)]
#[inline(always)]
fn parts_sums(ls: &[u64]) -> Vec<u64> {
    (0..=ls.len()).map(|x| ls.iter().skip(x).sum()).collect()
}
#[allow(dead_code)]
#[inline(always)]
fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    // your code
    let lc = l
        .clone()
        .into_iter()
        .map(|n| n.1)
        .skip(1)
        .fold(l[0].1, |c, n| (c).lcm(&n));
    //
    let division = l
        .into_iter()
        .map(|c| (lc / c.1) * c.0)
        .fold(0, |cu, ne| cu + ne);

    // Simply further
    if division % lc == 0 {
        Some((division / lc, 1))
    } else {
        // find the gcd
        let gc = lc.gcd(&division);
        if gc > 0 {
            Some((division / gc, lc / gc))
        } else {
            Some((division, lc))
        }
    }
}

#[allow(dead_code)]
#[inline(always)]
fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    // your code
    let mut result = Vec::new();

    for x in m..=n {
        //get divisors of each number
        let d = (1..=x).filter(|v| x % v == 0).map(|e| e * e).sum();
        //println!("{:?}, {}",&u,x);

        if is_perfect_square(d) {
            result.push((x, d));
        }
    }
    result
}
#[allow(dead_code)]
#[inline(always)]
fn is_perfect_square(int: u64) -> bool {
    let i: f64 = (int as f64).sqrt();
    (i - i.floor()) == 0.0_f64
}
#[allow(dead_code)]
#[inline(always)]
fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // your code
    // find lcm
    if l.len() < 1 {
        return vec![];
    }
    let lc = l
        .clone()
        .into_iter()
        .map(|n| n.1)
        .skip(1)
        .fold(l[0].1, |c, n| (c).lcm(&n));
    l.into_iter()
        .map(|x| ((lc / x.1) * x.0, lc))
        .map(|n| {
            if n.0 % 10 == 0 && n.1 % 10 == 0 {
                (n.0 / 10, n.1 / 10)
            } else {
                (n.0, n.1)
            }
        })
        .collect()
}

#[allow(dead_code)]
#[inline(always)]
fn my_fact(n: u32) -> f64 {
    (1..=n).fold(1.0, |c, n| (n as f64) * c)
}
#[allow(dead_code)]
#[inline(always)]
fn decomp(n: u32) -> Vec<u32> {
    let mut a = my_fact(n);
    let mut factors = vec![];
    for i in iter::range_inclusive(2f64, a) {
        while a % i == 0_f64 {
            factors.push(i as u32);

            a /= i;
        }
    }
    factors
}

