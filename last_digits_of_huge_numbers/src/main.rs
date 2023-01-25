#![allow(dead_code,unused_variables,unused_imports)]
// use Instant::now();
use std::time::Instant;

fn main() {
    let now = Instant::now();
  let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ];

         for test in tests { 
              println!( "{}",format_args!("{} ,{}", last_digit(&test.0), test.1));
         }
   println!("{:?}", now.elapsed());
}

fn last_digit(n:&[u64]) -> u64 {
     if n.is_empty() {
        return 1;
     }
     let  mut right_is_zero = false;
     let  mut right_bigger_than_2= false;
     let  mut right_mod4 =1;
      for i in (0..n.len()).rev() { 
 if  right_is_zero {
        right_mod4 = 1;
        right_is_zero = false;
        right_bigger_than_2 = false;

 }
 else { 

    right_mod4 =  if right_bigger_than_2 && (n[i]%4==2) {
        0
    } else {
        true_mod(n[i] as f64,right_mod4 as f64,4.0)

    };
            right_is_zero = n[i] ==0;
            right_bigger_than_2 =!right_is_zero && !(n[i]==1);
  }
      }
     
        if right_is_zero {
            return 1;
        }
        else { 
             true_mod(n[0] as f64,right_mod4 as f64,10.0)
        }
}

fn  true_mod(a:f64,n:f64,m:f64) -> u64 {
     let asn  =  (a%m) * (f64::powf(a%m,(n+3.0)%4.0)).round();
        (asn % m)  as u64
}



mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        let tests = vec![
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6)
        ];

        for test in tests {
            assert_eq!(last_digit(&test.0), test.1);
        }
    }
}