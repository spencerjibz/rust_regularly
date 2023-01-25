use std::time;
pub fn long_addition(right:&str,left:&str) -> String { 

    // make two vectors of each length
     let [rightarr, leftarr] = [right.chars(),left.chars()];
    
     // find the longest,   
     let mut longest:Vec<_> = if right.len() > left.len() { rightarr.clone().collect() } else if right.len() == left.len() { rightarr.clone().collect()} else {leftarr.clone().collect()};   
    
     let mut shortest:Vec<_>= if right.len() < left.len() { rightarr.collect() } else if right.len() == left.len() { leftarr.collect()} else {leftarr.collect()};
    
    // difference between longest and shortest
     let diff = longest.len() - shortest.len();
    
     // make the short one the same  size as the longest;
      if longest.len() == shortest.len() {
          longest.insert(0,'0');
          shortest.insert(0,'0');
      } else { 
    
      for _ in 0..diff {
          shortest.insert(0,'0');
      }
    
      // start addition       
    }
    
    longest.reverse();
    shortest.reverse();
          
    let mut result:Vec<char> = longest.clone();
   
     let   longest: Vec<u32> = longest.iter().map(|x|x.to_digit(10).unwrap()).collect();
    
     let    shortest:Vec<u32>= shortest.iter().map(|x|x.to_digit(10).unwrap()).collect();
     
     //println!("{:?},{:?}",shortest,longest);
     let mut remainder =0;
     for ind in 0.. longest.len() { 
           let sum = (remainder + longest[ind] + shortest[ind]).to_string();
           remainder = if sum.len()>1 {sum.chars().nth(0).unwrap().to_digit(10).unwrap()} else {0 };
    
           if sum.len()>2 { 
               result[ind]= sum.chars().nth(1).unwrap()
           }
           else {  
             result[ind] =sum.chars().nth(0).unwrap();
           }
     }
       result.reverse();
    
       // if  the length of result < length of longest, prefill with first digits of longest
        if remainder>0 {
      remainder.to_string().chars().for_each(|c| result.insert(0,c));
        }
        // remove  useless useless zeros
         if result[0]=='0' { 
           result.remove(0);
         }
    
        result.iter().collect()
    }
    
   /* pub fn long_add(a: &str, b: &str) -> Result<String,_> {
      let mut res  = String::new();
      let mut  c = 0.0;
         let mut  a_arr = a.chars().collect::<Vec<char>>();
           let mut  b_arr = b.chars().collect::<Vec<char>>();
         while a_arr.len()> 0 || b_arr.len()>0  { 
                   c  += (a_arr.pop()?.to_digit(10).ok()as f32).ceil() + (b_arr.pop()?.to_digit(10).ok()as f32).ceil();
                   
                    let g = res.parse::<f32>()?;
                     let ans = c%10.0 + g;
                       res = ans.to_string();
                    c =   if c > 9.0 { 1.0 } else { 0.0};
         }
     
       Ok(res)
      }
    
 */


 static ROMAN:[&str;13]= ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
  static ARABIC:[i32;13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
   
 
  pub fn to_roman(mut n: i32) -> String {
     let mut res = "".to_string();
    
      for i in 0..13 { 
         while n >= ARABIC[i] {
           
            res = res+ (ROMAN[i]);
             n -= ARABIC[i];
          
      }
   }
 res
  }

   pub fn from_roman (s:String) -> i32 { 

      // check for string length before processing
      if s.len() <1 {
         return -1;
      }
      let mut a =vec![];

       for c in s.chars() { 
           match c { 
               'I' => a.push(1),
               'V' => a.push(5),
               'X' => a.push(10),
               'L' => a.push(50),
               'C' => a.push(100),
               'D' => a.push(500),
               'M' => a.push(1000),
               _ => {
                  // ignore that char
               }
           }
       }
        let size = a.len();
    let mut res = a[size-1];
     for i in 0..size-1 {
         if a[i] >= a[i+1] {
             res += a[i];
         } else {
             res -= a[i];
         }
     }
   
      res

   }

   fn bech(count:i32) {
       for _ in 0..count {
    assert_eq!(to_roman(1000), "M");
    assert_eq!( to_roman(4), "IV");
    assert_eq!(to_roman(1), "I");
    assert_eq!(to_roman(1990), "MCMXC");
    assert_eq!(to_roman(2008), "MMVIII");

       }


    }
      fn  main() { 

        let now = time::Instant::now();
        // let ans = long_addition("823094582094385190384102934810293481029348123094818923749817", "234758927345982475298347523984572983472398457293847594193837");
       //  let ans2  = long_add("823094582094385190384102934810293481029348123094818923749817", "234758927345982475298347523984572983472398457293847594193837");
       bech(100000);
         println!("{:?}",now.elapsed());
     } 