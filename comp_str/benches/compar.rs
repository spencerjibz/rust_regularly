#![feature(test)] #![allow(dead_code,unused_imports)]
 
extern crate test;
 use test::Bencher;
 use test::black_box;
 use std::{iter,cmp::Ordering,cmp};
 use random_string::generate;
 use num_bigint::BigUint;
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
  fn chop<'a>(input: &mut &'a str, predicate: impl Fn(char) -> bool) -> &'a str {
    //         &str      |c: char| -> bool {...}
    let idx = input.find(|c| !predicate(c)).unwrap_or(input.len());// !predicate !|false
   	// "..." => ".." ".."
    let (head, tail) = input.split_at(idx); // mut start .. index , index -t end
    *input = tail; // 
    head // 
}

// 
fn chunkify(mut string: &str) -> impl Iterator<Item = &str> {
    iter::from_fn(move || {
        string.chars().next().map(|c| {
            if c.is_ascii_digit() {
              //				  |c: char| -> bool { c.is_ascii_digit()}	
                chop(&mut string, |c| c.is_ascii_digit())
            } else {
                chop(&mut string, |c| !c.is_ascii_digit()) // !ftrue
            }
        })
    })
}

fn compare_n(lhs: &str, rhs: &str) -> cmp::Ordering {
    match Ord::cmp(&lhs.len(), &rhs.len()) {
        cmp::Ordering::Less => cmp::Ordering::Less,
        cmp::Ordering::Equal => Ord::cmp(lhs, rhs), // here wiith  Ord trait, Partial (same |Ordering(line 47) return here )
        cmp::Ordering::Greater => cmp::Ordering::Greater,
    }
}

// "Item 123" "Item 3" => cmp::Ordering 

// "Item 4"
// "Item 123"
// "Item 531"

pub fn compare(lhs: &str, rhs: &str) -> cmp::Ordering {
    let mut lhs_chunks = chunkify(lhs);
    let mut rhs_chunks = chunkify(rhs); // (Some('item'), Some('1123'))
  

    loop {
        match (lhs_chunks.next(), rhs_chunks.next()) {
            (Some(lhs), Some(rhs)) => { // here 
                let ord = match (
                    lhs.contains(|c: char| c.is_ascii_digit()), // check if a digit is contained 
                    rhs.contains(|c: char| c.is_ascii_digit()),// here, ascii - 
                ) {
                    (true, true) => compare_n(lhs, rhs), // line by line
                    (false, false) => Ord::cmp(lhs, rhs), // here 
                    (true, false) => cmp::Ordering::Less,
                    (false, true) => cmp::Ordering::Greater,
                }; // here Optional of Some->

                match ord {
                    cmp::Ordering::Less => return cmp::Ordering::Less,
                    cmp::Ordering::Equal => continue, // next 
                    cmp::Ordering::Greater => return cmp::Ordering::Greater,
                }
            }
            (Some(_), None) => return cmp::Ordering::Greater,
            (None, Some(_)) => return cmp::Ordering::Less,
            (None, None) => return cmp::Ordering::Equal, // Equal (not digits )
        }
    }
}
#[bench]
fn mysol_bench(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    let charset = "abcdefghijklmnopqrstuvwxyz123457890";
    let mut v:Vec<String> = (0..10000).map(|i|generate(i,charset)).collect();
    b.iter(|| {
    
   // let mut v2 = vec!["Some123","alpha","James123"]; 
   
    v.sort_by(|a, b| black_box(compare_str(a, b))); 
    })
}

#[bench]
fn pass_bench(b: &mut Bencher) {
    let charset = "abcdefghijklmnopqrstuvwxyz123457890";
    let mut v:Vec<String> = (0..10000).map(|i|generate(i,charset)).collect();
    b.iter(|| {
        
        v.sort_by(|a, b| black_box(compare(a, b)));
    })
}
