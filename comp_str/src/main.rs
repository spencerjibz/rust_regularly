#![allow(dead_code,unused_variables,unused_imports)]
use std::{iter,cmp::Ordering, vec};
mod core_sol;
use core_sol::*;
use random_string::generate;
use num_bigint::BigUint;
fn main() {
    let now = std::time::Instant::now();
   // let charset = "abcdefghijklmnopqrstuvwxyz123457890";
   // let mut v:Vec<String> = (0..10000).map(|i|generate(i,charset)).collect();
    //et mut v = vec!["Some123","alpha","James123"]; 
   //println!("{:?}", v);
   // v.sort_by(|a, b| compare(a, b));   
    //println!("{:?}", v);
    //println!("{:?}", Username().fake::<String>());
    println!("{:?}", count_ones(12, 29));
    println!("{:?}", now.elapsed());
}


pub fn compare_str(right: &str, left: &str) -> Ordering {
    
   // two iters 
  
   // compare if both strings have digits, we compare the digits else we compare the strings
  match (has_digits(right),has_digits(left)) {
    (true,true) => {
      // first the index of first digit;
      let  right_index:usize = right.find(|c:char|c.is_ascii_digit()).unwrap_or(right.len());
        let  left_index:usize = left.find(|c:char|c.is_ascii_digit()).unwrap_or(left.len());
        // then we compare the digits
         let  (rest,d,)  = right.split_at(right_index);

            let  (rest2,d2)  = left.split_at(left_index);
           // println!("{}{}",rest, d2.len());
    
  //  convert digits to digits and  comp   . -> this is not necessary since the string is split at the first digit
// create safe numbers to computer 
/* 
let mut digits_right   =  String::new();
let mut digits_left =  String::new();


  d.chars().for_each(|c|{
    if c.is_ascii_digit() {
      digits_right.push(c);
    }
  });
  d2.chars().for_each(|c|{
    if c.is_ascii_digit() {
      digits_left.push(c);
    }
  });
 */

 //  
  let only_digit = d.chars().filter(|c|c.is_ascii_digit()).collect::<String>();
  let only_digit2 = d2.chars().filter(|c|c.is_ascii_digit()).collect::<String>();
  let num_right   = only_digit.parse::<BigUint>().unwrap();
  let num_left = only_digit2.parse::<BigUint>().unwrap();

   match num_right.cmp(&num_left) {
    Ordering::Equal => rest.cmp(&rest2),
      // if equal we compare the rest of the stri
    Ordering::Less => Ordering::Less,
    Ordering::Greater => Ordering::Greater,


   }
       
    },
    (true,false) => Ordering::Greater,
    (false,true) => Ordering::Less,
    (false,false) => right.cmp(left),
  }

}


// find digits and split them into

 fn has_digits(s:&str) -> bool { 
    s.contains(|c:char|c.is_ascii_digit())
 }
 // compare the digits




 /* 
 
 Given two numbers: 'left' and 'right' (1 <= 'left' <= 'right' <= 200000000000000) return sum of all '1' occurencies in binary representations of numbers between 'left' and 'right' (including both)

Example:
countOnes 4 7 should return 8, because:
4(dec) = 100(bin), which adds 1 to the result.
5(dec) = 101(bin), which adds 2 to the result.
6(dec) = 110(bin), which adds 2 to the result.
7(dec) = 111(bin), which adds 3 to the result.
So finally result equals 8.
WARNING: Segment may contain billion elements, to pass this kata, your solution cannot iterate through all numbers in the segment!
 
 */

fn count_ones(left:u64, right:u64) -> u64 {
    count(right) - count(left-1)
  }
fn count(n:u64) -> u64 {

  // find the lenght of the binary of left 
   let mut num = n as i64;
    let mut sum =0;
   while num >0  {
     let p = format!("{:b}", num).len()-1;
      let p2 = 1<<p;
      num -= p2;
      sum +=  p as i64 *(p2>>1)+1+num;
   }
   sum as u64
}
