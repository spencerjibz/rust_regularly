#![allow(dead_code)]

use std::time::Instant;
use cached::proc_macro::cached;
  fn main() {
    let now = Instant::now();
    println!(" ans = {} ", fib_iter(10000));
    println!("{:?}",now.elapsed())
  }
  
  // recursive fibonacci, O(2^n)
#[cached]
  fn fib (n: u64) -> u64 {
     if n == 0 || n == 1{
        return n
     }
      fib(n-1) + fib(n-2)
  }
  //  fibonacci iterative;

  fn fib_iter (n: u64) -> u64 {
    let mut first =0;
    let mut second = 0;
    let mut current = 1;
     let mut i  =1;
     while i < n { 
        first = second;
        second = current;
        current = first + second;
        i += 1;

     }
       return current
  }