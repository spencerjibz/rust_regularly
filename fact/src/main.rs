

use std::fmt::{Display, Formatter};
use std::time::Instant;
use itertools::Itertools;
use num::{bigint::BigUint,Zero,One};
#[derive(Clone, Debug)]
pub struct Factors {
    pub number: BigUint,
    pub result: Vec<BigUint>,
}
 
impl Factors {
    pub fn of(number: BigUint) -> Factors {
        let mut factors = Self {
            number: number.clone(),
            result: Vec::new(),
        };
 
        let big_2 = BigUint::from(2u8);
        let big_4 = BigUint::from(4u8);
 
        factors.check(&big_2);
        factors.check(&BigUint::from(3u8));
 
        let mut divisor = BigUint::from(5u8);
        while &divisor * &divisor <= factors.number {
            factors.check(&divisor);
            divisor += &big_2;
            factors.check(&divisor);
            divisor += &big_4;
        }
 
        if factors.number > BigUint::one() {
            factors.result.push(factors.number);
        }
 
        factors.number = number; // Restore the number
        factors
    }
 
    pub fn is_prime(&self) -> bool {
        self.result.len() == 1
    }
 
    fn check(&mut self, divisor: &BigUint) {
        while (&self.number % divisor).is_zero() {
            self.result.push(divisor.clone());
            self.number /= divisor;
        }
    }
}
 
impl Display for Factors {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut iter = self.result.iter();
 
        match iter.next() {
            None => write!(f, "[]"),
 
            Some(first) => {
                write!(f, "[{}", first)?;
                for next in iter {
                    write!(f, ", {}", next)?;
                }
 
                write!(f, "]")
            }
        }
    }
}
#[allow(dead_code)]
#[inline(always)]
fn print_factors(number: BigUint) {
    let factors = Factors::of(number);
 
    if factors.is_prime() {
        println!("{} -> {} (prime)", factors.number, factors);
    } else {
        println!("{} -> {}", factors.number, factors);
    }
}
 
#[allow(dead_code)]
#[inline(always)]
fn big_fact(n:i64) -> BigUint { 
   (1..=n).fold(BigUint::from(1u32),|c,n|c*BigUint::from(n as u32))


}
#[allow(dead_code)]
#[inline(always)]
fn decomp(n:i64) -> String{
  let factors:Vec<BigUint> = Factors::of(big_fact(n)).result;
   
  let s = factors.iter().unique().map(|e|{
   
    let  num = factors.iter().filter(|&el|el==e).count();
    //println!(" {} = {} elements",e,num);
    if num >1 {
       return format!("{}^{}",e,num)
    }
     format!("{}",e)
    
    
     }).join(" * ");
s
}
fn main() {
  let now = Instant::now();
  //println!("{:?}",Factors::of(( 9 as u64).into()).result);
   print_factors(big_fact(40000));
    // Find Mersenne primes
     
   println!("{:?}",now.elapsed())
   
}
 